use std::collections::BTreeSet;

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use syn::parse::{self, Parse};
use syn::Error;
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::Meta;
use syn::{self, Attribute, Field, NestedMeta, Variant};

pub enum DefaultExpr {
    Default,
    Path(syn::ExprPath),
    Lit(syn::Lit),
}

pub struct ModelAttr {
    pub deny_unknown_fields: bool,
}

impl ModelAttr {
    pub fn from_ast(input: &syn::DeriveInput) -> Result<ModelAttr, Error> {
        let mut deny_unknown_fields: Option<bool> = None;

        for attr in &input.attrs {
            if let Some(items) = get_meta_items(attr) {
                for meta_item in items {
                    match meta_item {
                        Meta(Word(ref w)) if w == "deny_unknown_fields" => {
                            deny_unknown_fields = Some(true);
                        }
                        _ => (),
                    }
                }
            }
        }

        let r = ModelAttr {
            deny_unknown_fields: deny_unknown_fields.unwrap_or(false),
        };
        Ok(r)
    }
}

pub struct FieldAttr {
    /// skip ser and deser
    pub ignore: bool,
    pub default: DefaultExpr,
    pub borrowed_lifetimes: BTreeSet<syn::Lifetime>,
}

impl FieldAttr {
    pub fn from_ast(field: &Field) -> Result<FieldAttr, Error> {
        let mut ignore: Option<bool> = None;
        let mut default: Option<DefaultExpr> = None;

        for attr in &field.attrs {
            if let Some(items) = get_meta_items(attr) {
                for meta_item in items {
                    match meta_item {
                        // #[toy(ignore)]
                        Meta(Word(ref w)) if w == "ignore" => {
                            ignore = Some(true);
                        }

                        // #[toy(default)]
                        Meta(Word(ref w)) if w == "default" => default = Some(DefaultExpr::Default),

                        // #[toy(default = ...)]
                        Meta(NameValue(ref m)) if m.ident == "default" => {
                            let path = parse_lit_into_lit(&m.ident, &m.lit)?;
                            default = Some(DefaultExpr::Lit(path));
                        }

                        // #[toy(default_expr = "...")]
                        Meta(NameValue(ref m)) if m.ident == "default_expr" => {
                            let path = parse_lit_into_expr_path(&m.ident, &m.lit)?;
                            default = Some(DefaultExpr::Path(path));
                        }
                        _ => (),
                    }
                }
            }
        }

        let mut lifetimes = BTreeSet::new();
        collect_lifetimes(&field.ty, &mut lifetimes);

        let r = FieldAttr {
            ignore: ignore.unwrap_or(false),
            default: default.unwrap_or(DefaultExpr::Default),
            borrowed_lifetimes: lifetimes,
        };
        Ok(r)
    }
}

pub struct VariantAttr {
    /// skip ser and deser
    pub ignore: bool,
}

impl VariantAttr {
    pub fn from_ast(variant: &Variant) -> Result<VariantAttr, Error> {
        let mut ignore: Option<bool> = None;

        for attr in &variant.attrs {
            if let Some(items) = get_meta_items(attr) {
                for meta_item in items {
                    match meta_item {
                        // #[toy(ignore)]
                        Meta(Word(ref w)) if w == "ignore" => {
                            ignore = Some(true);
                        }
                        _ => (),
                    }
                }
            }
        }

        let r = VariantAttr {
            ignore: ignore.unwrap_or(false),
        };
        Ok(r)
    }
}

fn collect_lifetimes(ty: &syn::Type, out: &mut BTreeSet<syn::Lifetime>) {
    match *ty {
        syn::Type::Slice(ref ty) => {
            collect_lifetimes(&ty.elem, out);
        }
        syn::Type::Array(ref ty) => {
            collect_lifetimes(&ty.elem, out);
        }
        syn::Type::Ptr(ref ty) => {
            collect_lifetimes(&ty.elem, out);
        }
        syn::Type::Reference(ref ty) => {
            out.extend(ty.lifetime.iter().cloned());
            collect_lifetimes(&ty.elem, out);
        }
        syn::Type::Tuple(ref ty) => {
            for elm in &ty.elems {
                collect_lifetimes(elm, out);
            }
        }
        syn::Type::Path(ref ty) => {
            if let Some(ref qself) = ty.qself {
                collect_lifetimes(&qself.ty, out);
            }
            for seg in &ty.path.segments {
                if let syn::PathArguments::AngleBracketed(ref bracketed) = seg.arguments {
                    for arg in &bracketed.args {
                        match *arg {
                            syn::GenericArgument::Lifetime(ref lifetime) => {
                                out.insert(lifetime.clone());
                            }
                            syn::GenericArgument::Type(ref ty) => {
                                collect_lifetimes(ty, out);
                            }
                            syn::GenericArgument::Binding(ref binding) => {
                                collect_lifetimes(&binding.ty, out);
                            }
                            syn::GenericArgument::Constraint(_) | syn::GenericArgument::Const(_) => {}
                        }
                    }
                }
            }
        }
        syn::Type::Paren(ref ty) => {
            collect_lifetimes(&ty.elem, out);
        }
        syn::Type::Group(ref ty) => {
            collect_lifetimes(&ty.elem, out);
        }
        syn::Type::Never(_)
        | syn::Type::TraitObject(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Verbatim(_)
        | syn::Type::BareFn(_) => {}
    }
}

fn get_meta_items(attr: &Attribute) -> Option<Vec<NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "toy" {
        match attr.interpret_meta() {
            Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_lit_into_expr_path(attr_name: &syn::Ident, lit: &syn::Lit) -> Result<syn::ExprPath, syn::Error> {
    let string = get_lit_str(attr_name, lit)?;
    parse_lit_str(string)
}

fn parse_lit_into_lit(attr_name: &syn::Ident, lit: &syn::Lit) -> Result<syn::Lit, syn::Error> {
    match *lit {
        syn::Lit::Str(ref v) => Ok(syn::Lit::Str(syn::LitStr::new(&v.value(), v.span()))),
        syn::Lit::Byte(ref v) => Ok(syn::Lit::Byte(syn::LitByte::new(v.value(), v.span()))),
        syn::Lit::ByteStr(ref v) => Ok(syn::Lit::ByteStr(syn::LitByteStr::new(&v.value(), v.span()))),
        syn::Lit::Char(ref v) => Ok(syn::Lit::Char(syn::LitChar::new(v.value(), v.span()))),
        syn::Lit::Int(ref v) => Ok(syn::Lit::Int(syn::LitInt::new(v.value(), v.suffix(), v.span()))),
        syn::Lit::Float(ref v) => Ok(syn::Lit::Float(syn::LitFloat::new(v.value(), v.suffix(), v.span()))),
        syn::Lit::Bool(ref v) => Ok(syn::Lit::Bool(syn::LitBool {
            value: v.value,
            span: v.span,
        })),
        _ => Err(Error::new_spanned(
            lit,
            format!(
                "{} attribute must to be str, byte, byteStr, char, int ,float, bool ",
                attr_name
            ),
        )),
    }
}

fn get_lit_str<'a>(attr_name: &syn::Ident, lit: &'a syn::Lit) -> Result<&'a syn::LitStr, syn::Error> {
    if let syn::Lit::Str(ref lit) = *lit {
        Ok(lit)
    } else {
        Err(Error::new_spanned(
            lit,
            format!("{} attribute must to be a string", attr_name),
        ))
    }
}

fn parse_lit_str<T>(s: &syn::LitStr) -> parse::Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

fn spanned_tokens(s: &syn::LitStr) -> parse::Result<TokenStream> {
    let stream = syn::parse_str(&s.value())?;
    Ok(respan_token_stream(stream, s.span()))
}

fn respan_token_stream(stream: TokenStream, span: Span) -> TokenStream {
    stream.into_iter().map(|token| respan_token_tree(token, span)).collect()
}

fn respan_token_tree(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(ref mut g) = token {
        *g = Group::new(g.delimiter(), respan_token_stream(g.stream().clone(), span));
    }
    token.set_span(span);
    token
}

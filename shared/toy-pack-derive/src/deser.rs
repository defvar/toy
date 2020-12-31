use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, TypeGenerics, WhereClause};

use super::ast::{model_from_ast, BorrowedLifetimes, Data, Field, Model, Style, Variant};
use super::attr::DefaultExpr;

struct DeserBlock {
    initialize_field: Vec<TokenStream>,
    deserialize_field: Vec<TokenStream>,
    deserialize_field_for_map: Vec<TokenStream>,
    unwrap_or: Vec<TokenStream>,
    construct_field: Vec<TokenStream>,
}

impl DeserBlock {
    fn new() -> DeserBlock {
        DeserBlock {
            initialize_field: Vec::new(),
            deserialize_field: Vec::new(),
            deserialize_field_for_map: Vec::new(),
            unwrap_or: Vec::new(),
            construct_field: Vec::new(),
        }
    }
}

struct IdentVisitorSource {
    pack_field_name: String,
    index: usize,
}

impl IdentVisitorSource {
    fn from_field(index: usize, field: &Field) -> IdentVisitorSource {
        IdentVisitorSource {
            pack_field_name: field.pack_field_name(),
            index,
        }
    }

    fn from_variant(index: usize, variant: &Variant) -> IdentVisitorSource {
        IdentVisitorSource {
            pack_field_name: variant.pack_field_name(),
            index,
        }
    }
}

pub fn derive_unpack_core(input: DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let model = match model_from_ast(&input) {
        Ok(v) => v,
        Err(e) => return Err(vec![e]),
    };

    let visitor_impl_block = match model.data {
        Data::Struct(s, ref f) => match struct_visitor_impl(&model, &model.input.ident, s, f, None)
        {
            Ok(v) => v,
            Err(e) => return Err(vec![e]),
        },
        Data::Enum(_) => match enum_visitor_impl(&model) {
            Ok(v) => v,
            Err(e) => return Err(vec![e]),
        },
    };

    let deser_impl_block = match deser_impl(&model) {
        Ok(v) => v,
        Err(e) => return Err(vec![e]),
    };

    let name = input.ident;

    // impl block wrap const, unique name. //
    let const_name = Ident::new(
        &format!(
            "_TOY_IMPL_DESER_FOR_{}",
            name.to_string().trim_start_matches("r#").to_owned()
        ),
        Span::call_site(),
    );
    let r = quote! {
        const #const_name: () = {
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            use toy_pack::deser as __deser;

            #visitor_impl_block

            #deser_impl_block
        };
    };

    Ok(r)
}

fn deser_impl(target: &Model) -> Result<TokenStream, syn::Error> {
    let visitor_name = visitor_name();
    let name = &target.input.ident;

    let (impl_generics, _, ty_generics, where_clause) =
        generics_and_lifetimes(&target.input, &target.borrowed);
    let toy_life = target.borrowed.toy_lifetime();

    let method = match target.data {
        Data::Enum(_) => quote!(
            deserializer.deserialize_enum(#visitor_name{
                marker: toy_pack::export::PhantomData,
                lifetime: toy_pack::export::PhantomData
            })
        ),
        Data::Struct(_, _) => quote!(
            deserializer.deserialize_struct(#visitor_name{
                marker: toy_pack::export::PhantomData,
                lifetime: toy_pack::export::PhantomData
            })
        ),
    };

    let r = quote! {
        impl #impl_generics __deser::Deserializable<#toy_life> for #name #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> toy_pack::export::Result<Self, D::Error>
                where D: __deser::Deserializer<#toy_life>
            {
                #method
            }
        }
    };
    Ok(r)
}

fn enum_visitor_impl(target: &Model) -> Result<TokenStream, syn::Error> {
    let (impl_generics, toy_ty_generics, ty_generics, where_clause) =
        generics_and_lifetimes(&target.input, &target.borrowed);
    let toy_life = target.borrowed.toy_lifetime();

    let variants = match &target.data {
        Data::Struct(_, _) => unreachable!(),
        Data::Enum(v) => v,
    };

    let visit_variants: Vec<TokenStream> = variants
        .iter()
        .enumerate()
        .flat_map(|(i, variant)| {
            let name = &target.input.ident;
            let variant_name = &variant.ident;
            let member_name = member_name(i);

            let q = match variant.style {
                Style::Unit => {
                    quote! {
                        (__Field::#member_name, _) => toy_pack::export::Ok(#name::#variant_name),
                    }
                }
                Style::Newtype => {
                    let ty = variant.fields.first().unwrap().ty;
                    quote! {
                        (__Field::#member_name, a) => toy_pack::export::Ok(#name::#variant_name(a.newtype_variant::<#ty>()?)),
                    }
                }
                Style::Tuple => {
                    let visitor_name = visitor_name();
                    let deserialize_fields = match struct_visitor_impl(target, name, Style::Tuple, &variant.fields, Some(variant_name)) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    quote! {
                        (__Field::#member_name, a) => {

                            #deserialize_fields

                            a.tuple_variant(#visitor_name{
                                marker: toy_pack::export::PhantomData,
                                lifetime: toy_pack::export::PhantomData
                            })
                        }
                    }
                }
                Style::Struct => unimplemented!(),
            };
            Ok(q)
        })
        .collect();

    let visitor_name = visitor_name();
    let enum_name = &target.input.ident;
    let enum_name_str = enum_name.to_string().trim_start_matches("r#").to_owned();
    let ident_sources = variants
        .iter()
        .enumerate()
        .map(|(i, v)| IdentVisitorSource::from_variant(i, v))
        .collect::<Vec<_>>();
    let identifier_impl = identifier_impl(&ident_sources);

    let q = quote! {
        struct #visitor_name #impl_generics #where_clause{
            marker: toy_pack::export::PhantomData<#enum_name #ty_generics>,
            lifetime: toy_pack::export::PhantomData<&#toy_life ()>,
        };

        impl #impl_generics __deser::Visitor<#toy_life> for #visitor_name #toy_ty_generics #where_clause {
            type Value = #enum_name #ty_generics;

            fn visit_enum<A>(self, data: A) -> toy_pack::export::Result<Self::Value, A::Error>
                where A: __deser::DeserializeVariantOps<#toy_life>
            {

                #identifier_impl

                match data.variant_identifier(__FieldVisitor)? {
                    #(#visit_variants)*
                    (_, _) => toy_pack::export::Err(__deser::Error::unknown_variant(#enum_name_str)),
                }
            }
        }
    };

    Ok(q)
}

fn struct_visitor_impl(
    target: &Model,
    ident: &Ident,
    style: Style,
    fields: &Vec<Field>,
    variant: Option<&Ident>,
) -> Result<TokenStream, syn::Error> {
    let (impl_generics, toy_ty_generics, ty_generics, where_clause) =
        generics_and_lifetimes(&target.input, &target.borrowed);
    let toy_life = target.borrowed.toy_lifetime();

    let block: DeserBlock = fields
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.attr.ignore)
        .map(|(i, field)| {
            (
                initialize_field(i, field),
                deserialize_field(i, field),
                deserialize_field_for_map(i, field),
                unwrap_or(i, field),
                match style {
                    Style::Tuple | Style::Newtype => construct_field_tuple(i),
                    Style::Struct => construct_field_struct(i, field),
                    Style::Unit => quote! {},
                },
            )
        })
        .fold(DeserBlock::new(), |mut acc, i| {
            acc.initialize_field.push(i.0);
            acc.deserialize_field.push(i.1);
            acc.deserialize_field_for_map.push(i.2);
            acc.unwrap_or.push(i.3);
            acc.construct_field.push(i.4);

            acc
        });

    let struct_name = ident;
    let visitor_name = visitor_name();
    let field_count = fields.iter().filter(|x| !x.attr.ignore).count();
    let initialize_field = block.initialize_field.clone();
    let initialize_field_for_map = block.initialize_field.clone();
    let deserialize_field = block.deserialize_field;
    let deserialize_field_for_map = block.deserialize_field_for_map;
    let unwarp_or = block.unwrap_or.clone();
    let unwarp_or_for_map = block.unwrap_or.clone();
    let construct_field = block.construct_field;
    let ident_sources = fields
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.attr.ignore)
        .map(|(i, v)| IdentVisitorSource::from_field(i, v))
        .collect::<Vec<_>>();
    let identifier_impl = identifier_impl(&ident_sources);

    let construct_result = match style {
        Style::Struct => {
            quote! {
                let r = #struct_name {
                     #(#construct_field)*
                 };
            }
        }
        Style::Unit => {
            quote! {
                let r = #struct_name;
            }
        }
        Style::Tuple | Style::Newtype => {
            let n = if let Some(v) = variant {
                quote!(#ident::#v)
            } else {
                quote!(#ident)
            };
            quote! {
                let r = #n(#(#construct_field)*);
            }
        }
    };

    let q = quote! {
        struct #visitor_name #impl_generics #where_clause{
            marker: toy_pack::export::PhantomData<#struct_name #ty_generics>,
            lifetime: toy_pack::export::PhantomData<&#toy_life ()>,
        };

        impl #impl_generics __deser::Visitor<#toy_life> for #visitor_name #toy_ty_generics #where_clause {
            type Value = #struct_name #ty_generics;

             fn visit_seq<A>(self, mut seq: A) -> toy_pack::export::Result<Self::Value, A::Error>
                 where A: __deser::DeserializeSeqOps<#toy_life>
             {

                 #(#initialize_field)*

                 for filed_idx in 0..#field_count {
                     match filed_idx {
                         #(#deserialize_field)*

                         _ => {
                             // TODO: discard value or error?
                             match seq.next::<__deser::discard::Discard>() {
                                 toy_pack::export::Ok(_) => (),
                                 toy_pack::export::Err(e) => return toy_pack::export::Err(e),
                             }
                         }
                     }
                 }

                 #(#unwarp_or)*

                 #construct_result

                 toy_pack::export::Ok(r)
            }

            fn visit_map<A>(self, mut map: A) -> toy_pack::export::Result<Self::Value, A::Error>
                where A: __deser::DeserializeMapOps<#toy_life>
            {
                #identifier_impl

                #(#initialize_field_for_map)*

                while let Some(key) = map.next_identifier(__FieldVisitor)? {
                    match key {
                        #(#deserialize_field_for_map)*

                        _ => {
                            // TODO: discard value or error?
                            match map.next_value::<__deser::discard::Discard>() {
                                toy_pack::export::Ok(_) => (),
                                toy_pack::export::Err(e) => return toy_pack::export::Err(e),
                            }
                        }
                    };
                }

                #(#unwarp_or_for_map)*

                #construct_result

                toy_pack::export::Ok(r)
            }
        }
    };

    Ok(q)
}

fn identifier_impl(source: &Vec<IdentVisitorSource>) -> TokenStream {
    let tokens = source
        .iter()
        .map(|x| {
            let indexed_name = member_name(x.index);
            let pack_field_name_str = &x.pack_field_name;
            let field_idx = x.index as u32;
            (
                quote! {
                    #indexed_name,
                },
                quote! {
                    #field_idx => toy_pack::export::Ok(__Field::#indexed_name),
                },
                quote! {
                    #pack_field_name_str => toy_pack::export::Ok(__Field::#indexed_name),
                },
            )
        })
        .fold(
            (
                Vec::<TokenStream>::new(),
                Vec::<TokenStream>::new(),
                Vec::<TokenStream>::new(),
            ),
            |mut acc, i| {
                acc.0.push(i.0);
                acc.1.push(i.1);
                acc.2.push(i.2);

                acc
            },
        );

    let field_def = tokens.0;
    let match_u32 = tokens.1;
    let match_str = tokens.2;

    quote! {
        #[allow(non_camel_case_types)]
        enum __Field { #(#field_def)* __unknown, }
        struct __FieldVisitor;

        impl <'toy> __deser::Visitor<'toy> for __FieldVisitor {
            type Value = __Field;

            fn visit_u32<E>(self, v: u32) -> toy_pack::export::Result<Self::Value, E> where E: __deser::Error {
                match v {
                    #(#match_u32)*
                    _ => toy_pack::export::Ok(__Field::__unknown)
                }
            }

            fn visit_str<E>(self, v: &str) -> toy_pack::export::Result<Self::Value, E> where E: __deser::Error {
                match v {
                    #(#match_str)*
                    _ => toy_pack::export::Ok(__Field::__unknown)
                }
            }
        }
    }
}

fn initialize_field(index: usize, field: &Field) -> TokenStream {
    let name = member_name(index);
    let ty = field.ty;
    quote! {
      let mut #name : Option<#ty> = None;
    }
}

fn deserialize_field(index: usize, field: &Field) -> TokenStream {
    let name = member_name(index);
    let ty = field.ty;
    let i = index;
    quote! {
      #i => {
          #name = match seq.next::<#ty>() {
              toy_pack::export::Ok(v) => v,
              toy_pack::export::Err(e) => return toy_pack::export::Err(e),
          }
      }
    }
}

fn deserialize_field_for_map(index: usize, field: &Field) -> TokenStream {
    let name = member_name(index);
    let ty = field.ty;
    let original_name = field.pack_field_name();
    quote! {
      __Field::#name => {
          if Option::is_some(&#name){
              return Err(__deser::Error::duplicate_field(#original_name));
          }
          #name = match map.next_value::<#ty>() {
              toy_pack::export::Ok(v) => Some(v),
              toy_pack::export::Err(e) => return toy_pack::export::Err(e),
          }
      }
    }
}

fn unwrap_or(index: usize, field: &Field) -> TokenStream {
    let name = member_name(index);
    let default_expr = match &field.attr.default {
        DefaultExpr::Default => quote!(toy_pack::export::Default::default()),
        DefaultExpr::Path(path) => quote!(#path()),
        DefaultExpr::Lit(lit) => quote!(#lit),
    };
    quote! {
      let #name = match #name {
        toy_pack::export::Some(v) => v,
        toy_pack::export::None => #default_expr,
      };
    }
}

fn construct_field_struct(index: usize, field: &Field) -> TokenStream {
    let indexed_name = member_name(index);
    let original_name = &field.member;
    quote! {
      #original_name: #indexed_name,
    }
}

fn construct_field_tuple(index: usize) -> TokenStream {
    let indexed_name = member_name(index);
    quote! {
      #indexed_name,
    }
}

fn member_name(i: usize) -> Ident {
    Ident::new(&format!("__field__{}", i), Span::call_site())
}

fn visitor_name() -> Ident {
    Ident::new(&"__Visitor", Span::call_site())
}

struct ToyImplGenerics<'a>(&'a DeriveInput, &'a BorrowedLifetimes);

impl<'a> ToTokens for ToyImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.0.generics.clone();
        let lifetimes = self.1;
        if let Some(lifetime) = lifetimes.toy_lifetime_def() {
            generics.params = Some(syn::GenericParam::Lifetime(lifetime))
                .into_iter()
                .chain(generics.params)
                .collect();
        }
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics.to_tokens(tokens);
    }
}

struct ToyTypeGenerics<'a>(&'a DeriveInput, &'a BorrowedLifetimes);

impl<'a> ToTokens for ToyTypeGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.0.generics.clone();
        if self.1.toy_lifetime_def().is_some() {
            let def = syn::LifetimeDef {
                attrs: Vec::new(),
                lifetime: syn::Lifetime::new("'toy", Span::call_site()),
                colon_token: None,
                bounds: Punctuated::new(),
            };
            generics.params = Some(syn::GenericParam::Lifetime(def))
                .into_iter()
                .chain(generics.params)
                .collect();
        }
        let (_, ty_generics, _) = generics.split_for_impl();
        ty_generics.to_tokens(tokens);
    }
}

fn generics_and_lifetimes<'a>(
    input: &'a DeriveInput,
    borrowed: &'a BorrowedLifetimes,
) -> (
    ToyImplGenerics<'a>,
    ToyTypeGenerics<'a>,
    TypeGenerics<'a>,
    Option<&'a WhereClause>,
) {
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    (
        ToyImplGenerics(input, borrowed),
        ToyTypeGenerics(input, borrowed),
        ty_generics,
        where_clause,
    )
}

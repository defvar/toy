use proc_macro2::Span;
use std::collections::BTreeSet;
use syn::punctuated::Punctuated;
use syn::{self, Fields};

use super::attr::{FieldAttr, ModelAttr, RenameExpr, VariantAttr};

pub struct Model<'a> {
    pub data: Data<'a>,
    pub borrowed: BorrowedLifetimes,
    pub input: &'a syn::DeriveInput,
    pub attr: ModelAttr,
}

impl<'a> Model<'a> {
    pub fn original_name(&self) -> String {
        self.input
            .ident
            .to_string()
            .trim_start_matches("r#")
            .to_owned()
    }
}

pub enum Data<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<Field<'a>>),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Style {
    /// Named fields.
    Struct,
    /// Many unnamed fields.
    Tuple,
    /// One unnamed field.
    Newtype,
    /// No fields.
    Unit,
}

#[allow(dead_code)]
pub struct Field<'a> {
    pub member: syn::Member,
    pub ty: &'a syn::Type,
    pub original: &'a syn::Field,
    pub attr: FieldAttr,
}

impl<'a> Field<'a> {
    pub fn pack_field_name(&self) -> String {
        match self.member {
            syn::Member::Named(ref ident) => match &self.attr.rename {
                RenameExpr::Default => ident.to_string().trim_start_matches("r#").to_owned(),
                RenameExpr::Lit(lit) => lit.to_string(),
            },
            syn::Member::Unnamed(ref i) => format!("{}", i.index),
        }
    }

    pub fn is_option_type(&self) -> bool {
        fn path_is_option(path: &syn::Path) -> bool {
            path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments.iter().next().unwrap().ident == "Option"
        }

        match self.ty {
            syn::Type::Path(typepath) if typepath.qself.is_none() => path_is_option(&typepath.path),
            _ => false,
        }
    }
}

#[allow(dead_code)]
pub struct Variant<'a> {
    pub ident: syn::Ident,
    pub attrs: VariantAttr,
    pub style: Style,
    pub fields: Vec<Field<'a>>,
    pub original: &'a syn::Variant,
}

impl<'a> Variant<'a> {
    pub fn pack_field_name(&self) -> String {
        match &self.attrs.rename {
            RenameExpr::Default => self.ident.to_string().trim_start_matches("r#").to_owned(),
            RenameExpr::Lit(lit) => lit.to_string(),
        }
    }
}

pub enum BorrowedLifetimes {
    Borrowed(BTreeSet<syn::Lifetime>),
    Static,
}

impl BorrowedLifetimes {
    pub fn toy_lifetime(&self) -> syn::Lifetime {
        match *self {
            BorrowedLifetimes::Borrowed(_) => syn::Lifetime::new("'toy", Span::call_site()),
            BorrowedLifetimes::Static => syn::Lifetime::new("'static", Span::call_site()),
        }
    }

    pub fn toy_lifetime_def(&self) -> Option<syn::LifetimeDef> {
        match *self {
            BorrowedLifetimes::Borrowed(ref bounds) => Some(syn::LifetimeDef {
                attrs: Vec::new(),
                lifetime: syn::Lifetime::new("'toy", Span::call_site()),
                colon_token: None,
                bounds: bounds.iter().cloned().collect(),
            }),
            BorrowedLifetimes::Static => None,
        }
    }
}

pub fn model_from_ast(input: &'_ syn::DeriveInput) -> Result<Model<'_>, syn::Error> {
    let data = match input.data {
        syn::Data::Enum(ref data) => Data::Enum(enum_from_ast(data)?),
        syn::Data::Struct(ref data) => {
            let (style, fields) = struct_from_ast(&data.fields)?;
            Data::Struct(style, fields)
        }
        syn::Data::Union(_) => unimplemented!(),
    };
    let borrowed = match data {
        Data::Struct(_, ref f) => borrowed_lifetimes(Box::new(f.iter())),
        Data::Enum(ref v) => {
            let fs = v.iter().flat_map(|variant| variant.fields.iter());
            borrowed_lifetimes(Box::new(fs))
        }
    };
    Ok(Model {
        data,
        borrowed,
        input: &input,
        attr: ModelAttr::from_ast(&input)?,
    })
}

fn struct_from_ast(fields: &'_ Fields) -> Result<(Style, Vec<Field<'_>>), syn::Error> {
    match *fields {
        Fields::Named(ref fields) => {
            // struct
            let a = field_from_ast(&fields.named)?;
            Ok((Style::Struct, a))
        }
        Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
            // new type struct
            Ok((Style::Newtype, field_from_ast(&fields.unnamed)?))
        }
        Fields::Unnamed(ref fields) => {
            // tuple struct
            Ok((Style::Tuple, field_from_ast(&fields.unnamed)?))
        }
        Fields::Unit => {
            //unit
            Ok((Style::Unit, vec![]))
        }
    }
}

fn enum_from_ast(data: &'_ syn::DataEnum) -> Result<Vec<Variant<'_>>, syn::Error> {
    let r = data
        .variants
        .iter()
        .map(|variant| match struct_from_ast(&variant.fields) {
            Ok((style, fields)) => match VariantAttr::from_ast(variant) {
                Ok(attrs) => Ok(Variant {
                    ident: variant.ident.clone(),
                    attrs,
                    style,
                    fields,
                    original: variant,
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        })
        .collect();
    r
}

fn field_from_ast(
    fields: &'_ Punctuated<syn::Field, Token![,]>,
) -> Result<Vec<Field<'_>>, syn::Error> {
    let r: Result<Vec<_>, _> = fields
        .iter()
        .enumerate()
        .map(|(i, field)| match FieldAttr::from_ast(field) {
            Ok(v) => Ok(Field {
                member: match field.ident {
                    Some(ref ident) => syn::Member::Named(ident.clone()),
                    None => syn::Member::Unnamed(i.into()),
                },
                ty: &field.ty,
                original: field,
                attr: v,
            }),
            Err(e) => Err(e),
        })
        .collect();
    r
}

fn borrowed_lifetimes<'a>(
    fields: Box<dyn Iterator<Item = &'a Field<'a>> + 'a>,
) -> BorrowedLifetimes {
    let mut lifetimes = BTreeSet::new();
    for field in fields {
        if !field.attr.ignore {
            lifetimes.extend(field.attr.borrowed_lifetimes.iter().cloned());
        }
    }
    if lifetimes.iter().any(|b| b.to_string() == "'static") {
        BorrowedLifetimes::Static
    } else {
        BorrowedLifetimes::Borrowed(lifetimes)
    }
}

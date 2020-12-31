use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

use super::ast::{model_from_ast, Data, Field, Model, Style};

pub fn derive_pack_core(input: DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let body = match body(&input) {
        Ok(v) => v,
        Err(e) => return Err(vec![e]),
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident;

    // main impl block //
    let impl_block = quote! {
        impl #impl_generics __ser::Serializable for #name #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> toy_pack::export::Result<S::Ok, S::Error>
                where S: __ser::Serializer,
            {
                #body
            }
        }
    };

    // impl block wrap const, unique name. //
    let const_name = Ident::new(
        &format!(
            "_TOY_IMPL_SER_FOR_{}",
            name.to_string().trim_start_matches("r#").to_owned()
        ),
        Span::call_site(),
    );
    let r = quote! {
        const #const_name: () = {
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            use toy_pack::ser as __ser;
            use toy_pack::ser::SerializeSeqOps as __SerializeSeqOps;
            use toy_pack::ser::SerializeStructOps as __SerializeStructOps;
            use toy_pack::ser::SerializeTupleVariantOps as __SerializeTupleVariantOps;

            #impl_block
        };
    };

    Ok(r)
}

fn body(input: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let model: Model = model_from_ast(&input)?;

    match &model.data {
        Data::Struct(_, _) => body_struct(&model),
        Data::Enum(_) => body_enum(&model),
    }
}

fn body_enum(target: &Model) -> Result<TokenStream, syn::Error> {
    let variants = match &target.data {
        Data::Struct(_, _) => unreachable!(),
        Data::Enum(v) => v,
    };

    let serialize_variants: Vec<TokenStream> = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let name = &target.input.ident;
            let variant_name = &variant.ident;
            let name_str = name.to_string().trim_start_matches("r#").to_owned();
            let variant_name_str = variant.pack_field_name();
            let idx = i as u32;
            match variant.style {
                Style::Unit => {
                    quote!(
                        #name::#variant_name => serializer.serialize_unit_variant(#name_str, #idx, #variant_name_str),
                    )
                }
                Style::Newtype => {
                    quote!(
                        #name::#variant_name(ref v) => serializer.serialize_newtype_variant(#name_str, #idx, #variant_name_str, v),
                    )
                }
                Style::Tuple => {
                    let len = serialize_length(&variant.fields);
                    let construct_fields: Vec<TokenStream> = variant
                        .fields.iter().enumerate()
                        .map(|(i, _)| {
                            let member_name = member_name(i);
                            quote! {
                                ref #member_name,
                            }
                        })
                        .collect();
                    let serialize_fields: Vec<TokenStream> = variant
                        .fields.iter().enumerate()
                        .filter(|(_, field)| !field.attr.ignore)
                        .map(|(i, _)| {
                            let member_name = member_name(i);
                            quote! {
                                ser.next(#member_name)?;
                            }
                        })
                        .collect();
                    quote!(
                        #name::#variant_name(#(#construct_fields)*) => {
                            let mut ser = serializer.serialize_tuple_variant(#name_str, #idx, #variant_name_str, #len)?;
                            #(#serialize_fields)*
                            ser.end()
                        },
                    )
                }
                Style::Struct => unimplemented!(),
            }
        })
        .collect();

    let q = quote! {
        match *self {
            #(#serialize_variants)*
        }
    };
    Ok(q)
}

fn body_struct(target: &Model) -> Result<TokenStream, syn::Error> {
    let (style, fields) = match &target.data {
        Data::Struct(s, f) => (s, f),
        Data::Enum(_) => unreachable!(),
    };
    let struct_name_str = target.original_name();
    let ignore_ser_if_none = target.attr.ignore_pack_if_none;
    match *style {
        Style::Struct => {
            let len = serialize_length(&fields);
            let serialize_fields: Vec<TokenStream> = fields
                .iter()
                .filter(|x| !x.attr.ignore)
                .map(|field| {
                    let name_str = field.pack_field_name();
                    let name = &field.member;
                    if ignore_ser_if_none && field.is_option_type() {
                        quote! {
                            if toy_pack::export::Option::is_some(&self.#name) {
                                serializer.field(#name_str, &self.#name)?;
                            }
                        }
                    } else {
                        quote! {
                            serializer.field(#name_str, &self.#name)?;
                        }
                    }
                })
                .collect();

            let q = quote! {
                let mut serializer = serializer.serialize_struct(#struct_name_str, #len)?;
                #(#serialize_fields)*
                serializer.end()
            };
            Ok(q)
        }
        Style::Unit => {
            let q = quote! {
                let mut serializer = serializer.serialize_struct(#struct_name_str, 0)?;
                serializer.end()
            };
            Ok(q)
        }
        Style::Tuple => unimplemented!(),
        Style::Newtype => unimplemented!(),
    }
}

fn member_name(i: usize) -> Ident {
    Ident::new(&format!("__field__{}", i), Span::call_site())
}

fn serialize_length(fields: &Vec<Field>) -> TokenStream {
    fields
        .iter()
        .filter(|x| !x.attr.ignore)
        .map(|_f| quote!(1))
        .fold(quote!(0), |sum, expr| quote!(#sum + #expr))
}

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

use super::ast::{model_from_ast, Data, Model, Style};

pub fn derive_schema_core(input: DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let body = match body(&input) {
        Ok(v) => v,
        Err(e) => return Err(vec![e]),
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident;

    // main impl block //
    let impl_block = quote! {
        impl #impl_generics __schema::Schema for #name #ty_generics #where_clause {
            fn scan<V>(name: &str, mut visitor: V) -> toy_pack::export::Result<V::Value, V::Error>
                where V: __schema::SchemaVisitor,
            {
                #body
            }
        }
    };

    // impl block wrap const, unique name. //
    let const_name = Ident::new(
        &format!(
            "_TOY_IMPL_SCHEMA_FOR_{}",
            name.to_string().trim_start_matches("r#").to_owned()
        ),
        Span::call_site(),
    );
    let r = quote! {
        const #const_name: () = {
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            use toy_pack::schema as __schema;
            use toy_pack::schema::StructVisitor as __StructVisitor;
            use toy_pack::schema::EnumVisitor as __EnumVisitor;
            use toy_pack::schema::TupleVariantVisitor as __TupleVariantVisitor;
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

    let enum_name = &target.input.ident;
    let name_str = enum_name.to_string().trim_start_matches("r#").to_owned();

    let schema_variants: Vec<TokenStream> = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let variant_name_str = variant.pack_field_name();
            match variant.style {
                Style::Unit => {
                    quote!(
                        enum_visitor.unit_variant(#name_str, #variant_name_str)?;
                    )
                }
                Style::Newtype => {
                    let member_name = member_name(i);
                    let tp = *&variant.fields.get(0).unwrap().ty;
                    quote!(
                        let #member_name = {
                            let mut tuple_visitor = enum_visitor.tuple_variant_visitor(#name_str, #variant_name_str)?;
                            tuple_visitor.tuple_variant_arg::<#tp>(#name_str, #variant_name_str, 0)?;
                            tuple_visitor.end()?
                        };
                        enum_visitor.variant(#name_str, #variant_name_str, #member_name)?;
                    )
                }
                Style::Tuple => {
                    let schema_fields: Vec<TokenStream> = variant
                        .fields.iter().enumerate()
                        .filter(|(_, field)| !field.attr.ignore)
                        .map(|(i, _)| {
                            let idx = i as u32;
                            let tp = *&variant.fields.get(0).unwrap().ty;
                            quote! {
                                tuple_visitor.tuple_variant_arg::<#tp>(#name_str, #variant_name_str, #idx)?;
                            }
                        })
                        .collect();
                    let member_name = member_name(i);
                    quote! {
                        let #member_name = {
                            let mut tuple_visitor = enum_visitor.tuple_variant_visitor(#name_str, #variant_name_str)?;
                            #(#schema_fields)*
                            tuple_visitor.end()?
                        };
                        enum_visitor.variant(#name_str, #variant_name_str, #member_name)?;
                    }
                }
                Style::Struct => unimplemented!(),
            }
        })
        .collect();

    let q = quote! {
        let mut enum_visitor = visitor.enum_visitor(name, #name_str)?;
        #(#schema_variants)*
        enum_visitor.end()
    };
    Ok(q)
}

fn body_struct(target: &Model) -> Result<TokenStream, syn::Error> {
    let (style, fields) = match &target.data {
        Data::Struct(s, f) => (s, f),
        Data::Enum(_) => unreachable!(),
    };
    let struct_name_str = target.original_name();
    match *style {
        Style::Struct => {
            let schema_fields: Vec<TokenStream> = fields
                .iter()
                .filter(|x| !x.attr.ignore)
                .map(|field| {
                    let name_str = field.pack_field_name();
                    let tp = *&field.ty;
                    quote! {
                        struct_visitor.field::<#tp>(#name_str)?;
                    }
                })
                .collect();

            let q = quote! {
                let mut struct_visitor = visitor.struct_visitor(#struct_name_str)?;
                #(#schema_fields)*
                struct_visitor.end()
            };
            Ok(q)
        }
        Style::Unit => unimplemented!(),
        Style::Tuple => unimplemented!(),
        Style::Newtype => unimplemented!(),
    }
}

fn member_name(i: usize) -> Ident {
    Ident::new(&format!("__field__{}", i), Span::call_site())
}

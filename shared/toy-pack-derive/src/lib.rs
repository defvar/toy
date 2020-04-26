#![recursion_limit = "256"]

extern crate proc_macro;
#[macro_use]
extern crate syn;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod ast;
mod attr;
mod deser;
mod ser;

#[proc_macro_derive(Pack, attributes(toy))]
pub fn derive_pack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ser::derive_pack_core(input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_derive(UnPack, attributes(toy))]
pub fn derive_unpack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    deser::derive_unpack_core(input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

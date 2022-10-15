#![feature(type_alias_impl_trait, error_generic_member_access, provide_any)]

mod client;
mod error;

pub use client::Client;
pub use error::RocksError;

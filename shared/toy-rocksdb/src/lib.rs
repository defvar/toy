#![feature(
    type_alias_impl_trait,
    error_generic_member_access,
    impl_trait_in_assoc_type
)]

mod client;
mod error;

pub use client::Client;
pub use error::RocksError;

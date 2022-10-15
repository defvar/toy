#![feature(type_alias_impl_trait, error_generic_member_access, provide_any)]

mod constants;
mod error;
mod store;

pub use store::{GLoggingStore, GLoggingStoreOps, GloggingStoreConnection};

#![feature(backtrace, type_alias_impl_trait)]

mod constants;
mod error;
mod store;

pub use store::{GLoggingStore, GLoggingStoreOps, GloggingStoreConnection};

#![feature(backtrace, min_type_alias_impl_trait)]

pub mod auth;
pub mod client;
mod common;
pub mod error;
pub mod noop;

pub use auth::Auth;
pub use client::ApiClient;
pub use noop::NoopApiClient;

#[cfg(feature = "http")]
pub mod http;

#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use toy_api;

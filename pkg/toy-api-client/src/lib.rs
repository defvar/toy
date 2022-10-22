#![feature(type_alias_impl_trait)]

pub mod client;
pub mod error;
pub mod noop;

pub use client::ApiClient;
pub use noop::NoopApiClient;

#[cfg(feature = "http")]
pub mod http;

#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use toy_api;
#[doc(hidden)]
pub use toy_api_http_common::auth::Auth;

//! HTTP Client Traits.
//!

#![feature(min_type_alias_impl_trait)]

mod client;
mod request;
mod response;

pub mod error;

pub use client::{HttpClient, NoopHttpClient};
pub use request::{NoopRequestBuilder, RequestBuilder};
pub use response::{ByteStream, NoopResponse, Response};

#[cfg(feature = "impl_reqwest")]
pub mod impl_reqwest;

#[doc(hidden)]
pub use bytes;
#[doc(hidden)]
pub use bytes::Bytes;
#[doc(hidden)]
pub use futures_core::Stream;
#[doc(hidden)]
pub use http;
#[doc(hidden)]
pub use http::{header, uri::InvalidUri, HeaderMap, Method, StatusCode, Uri, Version};

#[doc(hidden)]
#[cfg(feature = "impl_reqwest")]
pub use reqwest;

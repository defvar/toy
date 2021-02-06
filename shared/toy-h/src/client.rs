//! Trait for HTTP Client.
//!

use crate::request::NoopRequestBuilder;
use crate::RequestBuilder;
use http::{Method, Uri};
use std::fmt::Debug;

/// A Client to make outgoing HTTP requests.
pub trait HttpClient: Clone + Send + Sync + Debug {
    type Builder: RequestBuilder + Send;

    /// Start building a `GET` request to `Uri`.
    fn get<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    /// Start building a `POST` request to `Uri`.
    fn post<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    /// Start building a `PUT` request to `Uri`.
    fn put<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    /// Start building a `PATCH` request to `Uri`.
    fn patch<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    /// Start building a `DELETE` request to `Uri`.
    fn delete<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    /// Start building a `HEAD` request to `Uri`.
    fn head<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    /// Start building a request with the `Method` and `Uri`.
    fn request<T: Into<Uri>>(&self, method: http::Method, uri: T) -> Self::Builder;
}

#[derive(Clone, Debug)]
pub struct NoopHttpClient;

impl HttpClient for NoopHttpClient {
    type Builder = NoopRequestBuilder;

    fn get<T: Into<Uri>>(&self, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }

    fn post<T: Into<Uri>>(&self, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }

    fn put<T: Into<Uri>>(&self, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }

    fn patch<T: Into<Uri>>(&self, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }

    fn delete<T: Into<Uri>>(&self, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }

    fn head<T: Into<Uri>>(&self, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }

    fn request<T: Into<Uri>>(&self, _method: Method, _uri: T) -> Self::Builder {
        NoopRequestBuilder
    }
}

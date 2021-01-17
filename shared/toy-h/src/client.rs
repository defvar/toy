use crate::request::NoopRequestBuilder;
use crate::RequestBuilder;
use http::{Method, Uri};

pub trait HttpClient: Clone + Send + Sync {
    type Builder: RequestBuilder + Send;

    fn get<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    fn post<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    fn put<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    fn patch<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    fn delete<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

    fn head<T: Into<Uri>>(&self, uri: T) -> Self::Builder;

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

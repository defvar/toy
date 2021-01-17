use crate::error::HError;
use crate::response::{NoopResponse, Response};
use async_trait::async_trait;
use bytes::Bytes;
use http::header::{HeaderName, HeaderValue};
use http::HeaderMap;
use std::convert::TryFrom;

#[async_trait]
pub trait RequestBuilder {
    type Builder: RequestBuilder + Send;
    type Response: Response + Send;

    fn header<K, V>(self, key: K, value: V) -> Self::Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>;

    fn headers(self, headers: http::header::HeaderMap) -> Self::Builder;

    fn body<T: Into<Bytes>>(self, body: T) -> Self::Builder;

    async fn send(self) -> Result<Self::Response, HError>;
}

pub struct NoopRequestBuilder;

#[async_trait]
impl RequestBuilder for NoopRequestBuilder {
    type Builder = NoopRequestBuilder;
    type Response = NoopResponse;

    fn header<K, V>(self, _key: K, _value: V) -> Self::Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        NoopRequestBuilder
    }

    fn headers(self, _headers: HeaderMap<HeaderValue>) -> Self::Builder {
        NoopRequestBuilder
    }

    fn body<T: Into<Bytes>>(self, _body: T) -> Self::Builder {
        NoopRequestBuilder
    }

    async fn send(self) -> Result<Self::Response, HError> {
        Ok(NoopResponse::new())
    }
}

//! Implementation for reqwest.

use crate::error::HError;
use crate::response::{ByteStream, Response};
use crate::{HttpClient, RequestBuilder};
use async_trait::async_trait;
use bytes::Bytes;
use http::header::HeaderName;
use http::{HeaderMap, HeaderValue, Method, StatusCode, Uri, Version};
use std::convert::TryFrom;

#[derive(Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

pub struct ReqwestBuilder {
    raw: reqwest::RequestBuilder,
}

pub struct ReqwestResponse {
    raw: reqwest::Response,
}

impl ReqwestClient {
    pub fn from(client: reqwest::Client) -> ReqwestClient {
        ReqwestClient { client }
    }

    pub fn new() -> Result<ReqwestClient, HError> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| HError::error(e))?;
        Ok(ReqwestClient { client })
    }
}

impl HttpClient for ReqwestClient {
    type Builder = ReqwestBuilder;

    fn get<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.get(&uri.into().to_string()),
        }
    }

    fn post<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.post(&uri.into().to_string()),
        }
    }

    fn put<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.put(&uri.into().to_string()),
        }
    }

    fn patch<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.patch(&uri.into().to_string()),
        }
    }

    fn delete<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.delete(&uri.into().to_string()),
        }
    }

    fn head<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.head(&uri.into().to_string()),
        }
    }

    fn request<T: Into<Uri>>(&self, method: Method, uri: T) -> Self::Builder {
        ReqwestBuilder {
            raw: self.client.request(method, &uri.into().to_string()),
        }
    }
}

#[async_trait]
impl RequestBuilder for ReqwestBuilder {
    type Builder = ReqwestBuilder;
    type Response = ReqwestResponse;

    fn header<K, V>(self, key: K, value: V) -> Self::Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let raw = self.raw.header(key, value);
        Self { raw }
    }

    fn headers(self, headers: HeaderMap) -> Self::Builder {
        let raw = self.raw.headers(headers);
        Self { raw }
    }

    fn body<T: Into<Bytes>>(self, body: T) -> Self::Builder {
        let raw = self.raw.body(body.into());
        Self { raw }
    }

    async fn send(self) -> Result<Self::Response, HError> {
        self.raw
            .send()
            .await
            .map(|x| ReqwestResponse { raw: x })
            .map_err(|e| HError::error(e))
    }
}

#[async_trait]
impl Response for ReqwestResponse {
    fn status(&self) -> StatusCode {
        self.raw.status()
    }

    fn headers(&self) -> &HeaderMap {
        self.raw.headers()
    }

    async fn bytes(self) -> Result<Bytes, HError> {
        self.raw.bytes().await.map_err(|x| HError::error(x))
    }

    async fn chunk(&mut self) -> Result<Option<Bytes>, HError> {
        self.raw.chunk().await.map_err(|x| HError::error(x))
    }

    fn stream(self) -> ByteStream {
        ByteStream::from(self.raw.bytes_stream())
    }

    fn version(&self) -> Version {
        self.raw.version()
    }
}

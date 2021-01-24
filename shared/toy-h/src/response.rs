//! Trait for HTTP Response.
//!

use crate::error::HError;
use async_trait::async_trait;
use bytes::Bytes;
use core::task::Poll;
use futures_core::stream::BoxStream;
use futures_core::task::Context;
use futures_core::Stream;
use futures_util::TryStreamExt;
use http::{HeaderMap, StatusCode, Version};
use pin_project_lite::pin_project;
use std::pin::Pin;

/// A `Response`.
#[async_trait]
pub trait Response {
    /// Get the `StatusCode` of this `Response`.
    fn status(&self) -> StatusCode;

    /// Get the `Headers` of this `Response`.
    fn headers(&self) -> &HeaderMap;

    /// Get the full response body as `Bytes`.
    async fn bytes(self) -> Result<Bytes, HError>;

    /// Stream a chunk of the response body.
    ///
    /// When the response body has been exhausted, this will return `None`.
    async fn chunk(&mut self) -> Result<Option<Bytes>, HError>;

    /// Convert the response into a `Stream` of `Bytes` from the body.
    fn stream(self) -> ByteStream;

    /// Get the `Version` of this `Response`.
    fn version(&self) -> Version;
}

pub struct NoopResponse {
    headers: HeaderMap,
}

impl NoopResponse {
    pub fn new() -> Self {
        Self {
            headers: HeaderMap::new(),
        }
    }
}

#[async_trait]
impl Response for NoopResponse {
    fn status(&self) -> StatusCode {
        StatusCode::default()
    }

    fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    async fn bytes(self) -> Result<Bytes, HError> {
        Ok(Bytes::new())
    }

    async fn chunk(&mut self) -> Result<Option<Bytes>, HError> {
        Ok(None)
    }

    fn stream(self) -> ByteStream {
        unimplemented!()
    }

    fn version(&self) -> Version {
        Version::default()
    }
}

pin_project! {
    /// Wraped `Stream`.
    pub struct ByteStream {
        #[pin]
        inner: BoxStream<'static, Result<Bytes, Box<dyn std::error::Error + Send + Sync>>>,
    }
}

impl ByteStream {
    pub fn from<E>(stream: impl Stream<Item = Result<Bytes, E>> + Send + 'static) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            inner: Box::pin(stream.map_err(|e| e.into())),
        }
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes, HError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let r = futures_core::ready!(self.project().inner.poll_next(cx));

        match r {
            Some(Err(e)) => Poll::Ready(Some(Err(HError::error(e)))),
            Some(Ok(v)) => Poll::Ready(Some(Ok(v))),
            None => Poll::Ready(None),
        }
    }
}

//! Trait for HTTP Response.
//!

use crate::error::HError;
use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};

/// A `Response`.
#[async_trait]
pub trait Response {
    /// Get the `StatusCode` of this `Response`.
    fn status(&self) -> StatusCode;

    /// Get the `Headers` of this `Response`.
    fn headers(&self) -> &HeaderMap;

    /// Get the full response body as `Bytes`.
    async fn bytes(self) -> Result<Bytes, HError>;

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

    fn version(&self) -> Version {
        Version::default()
    }
}

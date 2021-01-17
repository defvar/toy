use crate::error::HError;
use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, StatusCode, Version};

#[async_trait]
pub trait Response {
    fn status(&self) -> StatusCode;

    fn headers(&self) -> &HeaderMap;

    async fn bytes(self) -> Result<Bytes, HError>;

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

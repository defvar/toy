use crate::client::MetricsClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{CommonPostResponse, PostOption};
use toy_api::metrics::Metrics;
use toy_api_http_common::{auth::Auth, request};
use toy_h::HttpClient;

static PATH: &'static str = "metrics";

#[derive(Debug, Clone)]
pub struct HttpMetricsClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpMetricsClient<T>
where
    T: HttpClient,
{
    pub fn new<P: Into<String>>(root: P, auth: Arc<Auth>, inner: T) -> Self {
        Self {
            root: root.into(),
            auth,
            inner,
        }
    }
}

#[async_trait]
impl<T> MetricsClient for HttpMetricsClient<T>
where
    T: HttpClient,
{
    async fn post(
        &self,
        v: Metrics,
        opt: PostOption,
    ) -> Result<CommonPostResponse, ApiClientError> {
        request::post(&self.inner, Some(&self.auth), &self.root, PATH, &v, opt)
            .await
            .map_err(|e| e.into())
    }
}

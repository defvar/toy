use crate::client::GraphClient;
use crate::error::ApiClientError;
use crate::Auth;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_api::graph::{Graph, GraphList};
use toy_h::HttpClient;

static PATH: &'static str = "graphs";

#[derive(Debug, Clone)]
pub struct HttpGraphClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpGraphClient<T>
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
impl<T> GraphClient for HttpGraphClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<GraphList, ApiClientError> {
        crate::http::list(&self.inner, &self.auth, &self.root, PATH, opt).await
    }

    async fn find(&self, key: String, opt: FindOption) -> Result<Option<Graph>, ApiClientError> {
        crate::http::find(&self.inner, &self.auth, &self.root, PATH, &key, opt).await
    }

    async fn put(&self, key: String, v: Graph, opt: PutOption) -> Result<(), ApiClientError> {
        crate::http::put(&self.inner, &self.auth, &self.root, PATH, &key, &v, opt).await
    }

    async fn delete(&self, key: String, opt: DeleteOption) -> Result<(), ApiClientError> {
        crate::http::delete(&self.inner, &self.auth, &self.root, PATH, &key, opt).await
    }
}

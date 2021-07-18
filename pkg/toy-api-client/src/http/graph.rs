use super::{common_headers, prepare_query};
use crate::client::GraphClient;
use crate::error::ApiClientError;
use crate::{common, Auth};
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_api::graph::{Graph, GraphList};
use toy_h::{HttpClient, RequestBuilder, Uri};

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
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let r = self.inner.get(uri).headers(h).send().await?;
        common::response(r, opt.format()).await
    }

    async fn find(&self, key: String, opt: FindOption) -> Result<Option<Graph>, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let r = self.inner.get(uri).headers(h).send().await?;
        common::response(r, opt.format()).await
    }

    async fn put(&self, key: String, v: Graph, opt: PutOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let body = common::encode(&v, opt.format())?;
        let r = self.inner.put(uri).headers(h).body(body).send().await?;
        common::no_response(r, opt.format()).await
    }

    async fn delete(&self, key: String, opt: DeleteOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let r = self.inner.delete(uri).headers(h).send().await?;
        common::no_response(r, opt.format()).await
    }
}

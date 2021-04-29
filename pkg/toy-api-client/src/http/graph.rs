use super::{common_headers, prepare_query};
use crate::client::GraphClient;
use crate::common;
use crate::error::ApiClientError;
use async_trait::async_trait;
use toy_api::graph::{DeleteOption, FindOption, GraphEntity, GraphsEntity, ListOption, PutOption};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone)]
pub struct HttpGraphClient<T> {
    root: String,
    inner: T,
}

impl<T> HttpGraphClient<T>
where
    T: HttpClient,
{
    pub fn new<P: Into<String>>(root: P, inner: T) -> Self {
        Self {
            root: root.into(),
            inner,
        }
    }
}

#[async_trait]
impl<T> GraphClient for HttpGraphClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<GraphsEntity, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<GraphsEntity>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<GraphEntity>, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<Option<GraphEntity>>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn put(&self, key: String, v: GraphEntity, opt: PutOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let body = common::encode(&v, opt.format())?;
        let _ = self
            .inner
            .put(uri)
            .headers(h)
            .body(body)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(())
    }

    async fn delete(&self, key: String, opt: DeleteOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/graphs/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let _ = self
            .inner
            .delete(uri)
            .headers(h)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(())
    }
}

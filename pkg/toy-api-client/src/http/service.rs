use super::{common_headers, prepare_query};
use crate::client::ServiceClient;
use crate::common;
use crate::error::ApiClientError;
use async_trait::async_trait;
use toy_api::services::{
    DeleteOption, FindOption, ListOption, PutOption, ServiceSpec, ServiceSpecList,
};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone)]
pub struct HttpServiceClient<T> {
    root: String,
    inner: T,
}

impl<T> HttpServiceClient<T>
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
impl<T> ServiceClient for HttpServiceClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<ServiceSpecList, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/services?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<ServiceSpecList>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<ServiceSpec>, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/services/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<Option<ServiceSpec>>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn put(&self, key: String, v: ServiceSpec, opt: PutOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/services/{}?{}", self.root, key, query).parse::<Uri>()?;
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
        let uri = format!("{}/services/{}?{}", self.root, key, query).parse::<Uri>()?;
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

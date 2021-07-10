use super::{common_headers, prepare_query};
use crate::client::SupervisorClient;
use crate::error::ApiClientError;
use crate::{common, Auth};
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_api::supervisors::{Supervisor, SupervisorList};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone)]
pub struct HttpSupervisorClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpSupervisorClient<T>
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
impl<T> SupervisorClient for HttpSupervisorClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<SupervisorList, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/supervisors?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<SupervisorList>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<Supervisor>, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/supervisors/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<Option<Supervisor>>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn put(&self, key: String, v: Supervisor, opt: PutOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/supervisors/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
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
        let uri = format!("{}/supervisors/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
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

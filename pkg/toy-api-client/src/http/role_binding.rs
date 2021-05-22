use super::{common_headers, prepare_query};
use crate::client::RoleBindingClient;
use crate::common;
use crate::error::ApiClientError;
use async_trait::async_trait;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_api::role_binding::{RoleBinding, RoleBindingList};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone)]
pub struct HttpRoleBindingClient<T> {
    root: String,
    inner: T,
}

impl<T> HttpRoleBindingClient<T>
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
impl<T> RoleBindingClient for HttpRoleBindingClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<RoleBindingList, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/roleBindings?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<RoleBindingList>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<RoleBinding>, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/roleBindings/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<Option<RoleBinding>>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn put(&self, key: String, v: RoleBinding, opt: PutOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/roleBindings/{}?{}", self.root, key, query).parse::<Uri>()?;
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
        let uri = format!("{}/roleBindings/{}?{}", self.root, key, query).parse::<Uri>()?;
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

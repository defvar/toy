use crate::client::RoleClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{CommonPutResponse, DeleteOption, FindOption, ListOption, PutOption};
use toy_api::role::{Role, RoleList};
use toy_api_http_common::{auth::Auth, request};
use toy_h::HttpClient;

static PATH: &'static str = "rbac/roles";

#[derive(Debug, Clone)]
pub struct HttpRoleClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpRoleClient<T>
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
impl<T> RoleClient for HttpRoleClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<RoleList, ApiClientError> {
        request::list(&self.inner, Some(&self.auth), &self.root, PATH, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn find(&self, key: String, opt: FindOption) -> Result<Option<Role>, ApiClientError> {
        request::find(&self.inner, Some(&self.auth), &self.root, PATH, &key, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn put(
        &self,
        key: String,
        v: Role,
        opt: PutOption,
    ) -> Result<CommonPutResponse, ApiClientError> {
        request::put(
            &self.inner,
            Some(&self.auth),
            &self.root,
            PATH,
            &key,
            &v,
            opt,
        )
        .await
        .map_err(|e| e.into())
    }

    async fn delete(&self, key: String, opt: DeleteOption) -> Result<(), ApiClientError> {
        request::delete(&self.inner, Some(&self.auth), &self.root, PATH, &key, opt)
            .await
            .map_err(|e| e.into())
    }
}

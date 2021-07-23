use crate::client::RoleBindingClient;
use crate::error::ApiClientError;
use crate::Auth;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_api::role_binding::{RoleBinding, RoleBindingList};
use toy_h::HttpClient;

static PATH: &'static str = "rbac/roleBindings";

#[derive(Debug, Clone)]
pub struct HttpRoleBindingClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpRoleBindingClient<T>
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
impl<T> RoleBindingClient for HttpRoleBindingClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ListOption) -> Result<RoleBindingList, ApiClientError> {
        crate::http::list(&self.inner, &self.auth, &self.root, PATH, opt).await
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<RoleBinding>, ApiClientError> {
        crate::http::find(&self.inner, &self.auth, &self.root, PATH, &key, opt).await
    }

    async fn put(&self, key: String, v: RoleBinding, opt: PutOption) -> Result<(), ApiClientError> {
        crate::http::put(&self.inner, &self.auth, &self.root, PATH, &key, &v, opt).await
    }

    async fn delete(&self, key: String, opt: DeleteOption) -> Result<(), ApiClientError> {
        crate::http::delete(&self.inner, &self.auth, &self.root, PATH, &key, opt).await
    }
}

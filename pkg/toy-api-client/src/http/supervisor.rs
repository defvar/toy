use crate::client::SupervisorClient;
use crate::error::ApiClientError;
use crate::Auth;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{DeleteOption, FindOption, PutOption};
use toy_api::supervisors::{Supervisor, SupervisorList, SupervisorListOption};
use toy_h::HttpClient;

static PATH: &'static str = "supervisors";

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
    async fn list(&self, opt: SupervisorListOption) -> Result<SupervisorList, ApiClientError> {
        crate::http::list_with_opt(&self.inner, &self.auth, &self.root, PATH, opt).await
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<Supervisor>, ApiClientError> {
        crate::http::find(&self.inner, &self.auth, &self.root, PATH, &key, opt).await
    }

    async fn put(&self, key: String, v: Supervisor, opt: PutOption) -> Result<(), ApiClientError> {
        crate::http::put(&self.inner, &self.auth, &self.root, PATH, &key, &v, opt).await
    }

    async fn delete(&self, key: String, opt: DeleteOption) -> Result<(), ApiClientError> {
        crate::http::delete(&self.inner, &self.auth, &self.root, PATH, &key, opt).await
    }
}

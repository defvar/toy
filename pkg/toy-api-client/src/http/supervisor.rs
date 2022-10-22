use crate::client::SupervisorClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{CommonPutResponse, DeleteOption, FindOption, PostOption, PutOption};
use toy_api::supervisors::{
    Supervisor, SupervisorBeatResponse, SupervisorList, SupervisorListOption,
};
use toy_api_http_common::{auth::Auth, request};
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
        request::list(&self.inner, Some(&self.auth), &self.root, PATH, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn find(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<Option<Supervisor>, ApiClientError> {
        request::find(&self.inner, Some(&self.auth), &self.root, PATH, &key, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn put(
        &self,
        key: String,
        v: Supervisor,
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

    async fn beat(&self, key: &str) -> Result<SupervisorBeatResponse, ApiClientError> {
        let path = format!("{}/{}/beat", PATH, key);
        request::post(
            &self.inner,
            Some(&self.auth),
            &self.root,
            &path,
            &(),
            PostOption::new(),
        )
        .await
        .map_err(|e| e.into())
    }
}

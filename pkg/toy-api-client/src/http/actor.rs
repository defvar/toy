use crate::client::ActorClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::actors::{Actor, ActorBeatResponse, ActorList, ActorListOption};
use toy_api::common::{CommonPutResponse, DeleteOption, FindOption, PostOption, PutOption};
use toy_api_http_common::{auth::Auth, request};
use toy_h::HttpClient;

static PATH: &'static str = "actors";

#[derive(Debug, Clone)]
pub struct HttpActorClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpActorClient<T>
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
impl<T> ActorClient for HttpActorClient<T>
where
    T: HttpClient,
{
    async fn list(&self, opt: ActorListOption) -> Result<ActorList, ApiClientError> {
        request::list(&self.inner, Some(&self.auth), &self.root, PATH, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn find(&self, key: String, opt: FindOption) -> Result<Option<Actor>, ApiClientError> {
        request::find(&self.inner, Some(&self.auth), &self.root, PATH, &key, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn put(
        &self,
        key: String,
        v: Actor,
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

    async fn beat(&self, key: &str) -> Result<ActorBeatResponse, ApiClientError> {
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

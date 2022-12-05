use crate::client::TaskClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::{CommonPostResponse, FindOption, PostOption};
use toy_api::graph::Graph;
use toy_api::task::{
    FinishResponse, PendingResult, TaskEvent, TaskEventList, TaskListOption, Tasks,
};
use toy_api_http_common::{auth::Auth, request};
use toy_core::prelude::TaskId;
use toy_h::{HttpClient, RequestBuilder, Uri};

static PATH: &'static str = "tasks";

#[derive(Debug, Clone)]
pub struct HttpTaskClient<T> {
    root: String,
    auth: Arc<Auth>,
    inner: T,
}

impl<T> HttpTaskClient<T>
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
impl<T> TaskClient for HttpTaskClient<T>
where
    T: HttpClient,
{
    async fn post(&self, v: Graph, opt: PostOption) -> Result<PendingResult, ApiClientError> {
        request::post(&self.inner, Some(&self.auth), &self.root, PATH, &v, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn finish(&self, key: TaskId, opt: PostOption) -> Result<FinishResponse, ApiClientError> {
        let path = format!("{}/{}/finish", PATH, key);
        request::post(&self.inner, Some(&self.auth), &self.root, &path, &(), opt)
            .await
            .map_err(|e| e.into())
    }

    async fn list(&self, opt: TaskListOption) -> Result<Tasks, ApiClientError> {
        request::list(&self.inner, Some(&self.auth), &self.root, PATH, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn find_event(
        &self,
        key: String,
        opt: FindOption,
    ) -> Result<TaskEventList, ApiClientError> {
        let query = request::prepare_query(&opt)?;
        let uri = format!("{}/tasks/events/{}?{}", self.root, key, query).parse::<Uri>()?;
        let h = request::common_headers(opt.format(), Some(&self.auth));
        let r = self.inner.get(uri).headers(h).send().await?;
        request::decode(r, opt.format()).await.map_err(|e| e.into())
    }

    async fn post_event(
        &self,
        v: Vec<TaskEvent>,
        opt: PostOption,
    ) -> Result<CommonPostResponse, ApiClientError> {
        let path = format!("{}/events", PATH);
        request::post(&self.inner, Some(&self.auth), &self.root, &path, &v, opt)
            .await
            .map_err(|e| e.into())
    }
}

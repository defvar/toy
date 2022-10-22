use crate::client::TaskClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::common::PostOption;
use toy_api::graph::Graph;
use toy_api::task::{LogOption, PendingResult, TaskListOption, TaskLog, Tasks};
use toy_api_http_common::{auth::Auth, request};
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

    async fn list(&self, opt: TaskListOption) -> Result<Tasks, ApiClientError> {
        request::list(&self.inner, Some(&self.auth), &self.root, PATH, opt)
            .await
            .map_err(|e| e.into())
    }

    async fn log(&self, key: String, opt: LogOption) -> Result<TaskLog, ApiClientError> {
        let query = request::prepare_query(&opt)?;
        let uri = format!("{}/tasks/{}/log?{}", self.root, key, query).parse::<Uri>()?;
        let h = request::common_headers(opt.format(), Some(&self.auth));
        let r = self.inner.get(uri).headers(h).send().await?;
        request::decode(r, opt.format()).await.map_err(|e| e.into())
    }
}

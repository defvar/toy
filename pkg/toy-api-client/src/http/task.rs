use crate::client::TaskClient;
use crate::error::ApiClientError;
use crate::http::common::{common_headers, prepare_query};
use crate::{common, Auth};
use async_trait::async_trait;
use std::sync::Arc;
use toy_api::graph::Graph;
use toy_api::task::{LogOption, PendingResult, PostOption, TaskListOption, TaskLog, Tasks};
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
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let body = common::encode(&v, opt.format())?;
        let r = self.inner.post(uri).headers(h).body(body).send().await?;
        common::response(r, opt.format()).await
    }

    async fn list(&self, opt: TaskListOption) -> Result<Tasks, ApiClientError> {
        crate::http::list_with_opt(&self.inner, &self.auth, &self.root, PATH, opt).await
    }

    async fn log(&self, key: String, opt: LogOption) -> Result<TaskLog, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks/{}/log?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let r = self.inner.get(uri).headers(h).send().await?;
        common::response(r, opt.format()).await
    }
}

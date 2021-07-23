use crate::client::TaskClient;
use crate::error::ApiClientError;
use crate::http::common::{common_headers, prepare_query};
use crate::{common, Auth};
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::StreamExt;
use std::sync::Arc;
use toy_api::common::Format;
use toy_api::error::ErrorMessage;
use toy_api::graph::Graph;
use toy_api::task::{
    AllocateOption, AllocateRequest, AllocateResponse, ListOption, LogOption, PendingsEntity,
    PostOption, TaskLogEntity, TasksEntity, WatchOption,
};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

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
    type WatchStream = impl Stream<Item = Result<PendingsEntity, ApiClientError>>;

    async fn watch(&self, opt: WatchOption) -> Result<Self::WatchStream, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks/watch?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let response = self.inner.get(uri).headers(h).send().await?;

        if response.status().is_success() {
            let stream = response.stream().map(move |bytes| match bytes {
                Ok(v) => common::decode::<PendingsEntity>(&v, opt.format()).map_err(|e| e.into()),
                Err(e) => Err(e.into()),
            });
            Ok(stream)
        } else {
            let bytes = response.bytes().await?;
            let r = common::decode::<ErrorMessage>(&bytes, Some(Format::Json))?;
            Err(r.into())
        }
    }

    async fn allocate(
        &self,
        key: String,
        req: AllocateRequest,
        opt: AllocateOption,
    ) -> Result<AllocateResponse, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks/{}/allocate?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let body = common::encode(&req, opt.format())?;
        let r = self.inner.post(uri).headers(h).body(body).send().await?;
        common::response(r, opt.format()).await
    }

    async fn post(&self, v: Graph, opt: PostOption) -> Result<(), ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let body = common::encode(&v, opt.format())?;
        let r = self.inner.post(uri).headers(h).body(body).send().await?;
        common::no_response(r, opt.format()).await
    }

    async fn list(&self, opt: ListOption) -> Result<TasksEntity, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let r = self.inner.get(uri).headers(h).send().await?;
        common::response(r, opt.format()).await
    }

    async fn log(&self, key: String, opt: LogOption) -> Result<TaskLogEntity, ApiClientError> {
        let query = prepare_query(&opt)?;
        let uri = format!("{}/tasks/{}/log?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format(), &self.auth);
        let r = self.inner.get(uri).headers(h).send().await?;
        common::response(r, opt.format()).await
    }
}

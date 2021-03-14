use crate::client::TaskClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::StreamExt;
use toy_api::graph::GraphEntity;
use toy_api::task::{
    ListOption, LogOption, PendingsEntity, PostOption, TaskLogEntity, TasksEntity, WatchOption,
};
use toy_h::{header::HeaderValue, header::CONTENT_TYPE, HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug)]
pub struct HttpTaskClient<T> {
    root: String,
    inner: T,
}

impl<T> HttpTaskClient<T>
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
impl<T> TaskClient for HttpTaskClient<T>
where
    T: HttpClient,
{
    type WatchStream = impl Stream<Item = Result<PendingsEntity, ApiClientError>>;

    async fn watch(&self, _opt: WatchOption) -> Result<Self::WatchStream, ApiClientError> {
        let uri = format!("{}/tasks/watch", self.root).parse::<Uri>()?;
        let stream = self
            .inner
            .post(uri)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()
            .await?
            .stream()
            .map(|bytes| match bytes {
                Ok(v) => toy_pack_json::unpack::<PendingsEntity>(&v).map_err(|e| e.into()),
                Err(e) => Err(e.into()),
            });
        Ok(stream)
    }

    async fn post(&self, v: GraphEntity, _opt: PostOption) -> Result<(), ApiClientError> {
        let uri = format!("{}/tasks", self.root).parse::<Uri>()?;
        let body = toy_pack_json::pack(&v)?;
        let _ = self
            .inner
            .post(uri)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(())
    }

    async fn list(&self, _opt: ListOption) -> Result<TasksEntity, ApiClientError> {
        let uri = format!("{}/tasks", self.root).parse::<Uri>()?;
        let bytes = self.inner.get(uri).send().await?.bytes().await?;
        let r = toy_pack_json::unpack::<TasksEntity>(&bytes)?;
        Ok(r)
    }

    async fn log(&self, key: String, _opt: LogOption) -> Result<TaskLogEntity, ApiClientError> {
        let uri = format!("{}/tasks/{}/log", self.root, key).parse::<Uri>()?;
        let bytes = self.inner.get(uri).send().await?.bytes().await?;
        let r = toy_pack_json::unpack::<TaskLogEntity>(&bytes)?;
        Ok(r)
    }
}

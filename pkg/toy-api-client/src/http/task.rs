use super::common_headers;
use crate::client::TaskClient;
use crate::common;
use crate::error::ApiClientError;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::StreamExt;
use toy_api::graph::GraphEntity;
use toy_api::task::{
    AllocateOption, AllocateRequest, AllocateResponse, ListOption, LogOption, PendingsEntity,
    PostOption, TaskLogEntity, TasksEntity, WatchOption,
};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone)]
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

    async fn watch(&self, opt: WatchOption) -> Result<Self::WatchStream, ApiClientError> {
        let query = toy_pack_urlencoded::pack_to_string(&opt)?;
        let uri = format!("{}/tasks/watch?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let stream = self
            .inner
            .get(uri)
            .headers(h)
            .send()
            .await?
            .stream()
            .map(move |bytes| match bytes {
                Ok(v) => common::decode::<PendingsEntity>(&v, opt.format()).map_err(|e| e.into()),
                Err(e) => Err(e.into()),
            });
        Ok(stream)
    }

    async fn allocate(
        &self,
        key: String,
        req: AllocateRequest,
        opt: AllocateOption,
    ) -> Result<AllocateResponse, ApiClientError> {
        let query = toy_pack_urlencoded::pack_to_string(&opt)?;
        let uri = format!("{}/tasks/{}/allocate?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let body = common::encode(&req, opt.format())?;
        let bytes = self
            .inner
            .post(uri)
            .headers(h)
            .body(body)
            .send()
            .await?
            .bytes()
            .await?;
        let r = common::decode::<AllocateResponse>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn post(&self, v: GraphEntity, opt: PostOption) -> Result<(), ApiClientError> {
        let query = toy_pack_urlencoded::pack_to_string(&opt)?;
        let uri = format!("{}/tasks?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let body = common::encode(&v, opt.format())?;
        let _ = self
            .inner
            .post(uri)
            .headers(h)
            .body(body)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(())
    }

    async fn list(&self, opt: ListOption) -> Result<TasksEntity, ApiClientError> {
        let query = toy_pack_urlencoded::pack_to_string(&opt)?;
        let uri = format!("{}/tasks?{}", self.root, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<TasksEntity>(&bytes, opt.format())?;
        Ok(r)
    }

    async fn log(&self, key: String, opt: LogOption) -> Result<TaskLogEntity, ApiClientError> {
        let query = toy_pack_urlencoded::pack_to_string(&opt)?;
        let uri = format!("{}/tasks/{}/log?{}", self.root, key, query).parse::<Uri>()?;
        let h = common_headers(opt.format());
        let bytes = self.inner.get(uri).headers(h).send().await?.bytes().await?;
        let r = common::decode::<TaskLogEntity>(&bytes, opt.format())?;
        Ok(r)
    }
}

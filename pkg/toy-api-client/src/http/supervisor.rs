use crate::client::SupervisorClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use toy_api::supervisors::{
    DeleteOption, FindOption, ListOption, PutOption, Supervisor, Supervisors,
};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone)]
pub struct HttpSupervisorClient<T> {
    root: String,
    inner: T,
}

impl<T> HttpSupervisorClient<T>
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
impl<T> SupervisorClient for HttpSupervisorClient<T>
where
    T: HttpClient,
{
    async fn list(&self, _opt: ListOption) -> Result<Supervisors, ApiClientError> {
        let uri = format!("{}/supervisors", self.root).parse::<Uri>()?;
        let bytes = self.inner.get(uri).send().await?.bytes().await?;
        let r = toy_pack_json::unpack::<Supervisors>(&bytes)?;
        Ok(r)
    }

    async fn find(
        &self,
        key: String,
        _opt: FindOption,
    ) -> Result<Option<Supervisor>, ApiClientError> {
        let uri = format!("{}/supervisors/{}", self.root, key).parse::<Uri>()?;
        let bytes = self.inner.get(uri).send().await?.bytes().await?;
        let r = toy_pack_json::unpack::<Option<Supervisor>>(&bytes)?;
        Ok(r)
    }

    async fn put(&self, key: String, v: Supervisor, _opt: PutOption) -> Result<(), ApiClientError> {
        let uri = format!("{}/supervisors/{}", self.root, key).parse::<Uri>()?;
        let body = toy_pack_json::pack(&v)?;
        let _ = self.inner.put(uri).body(body).send().await?.bytes().await?;
        Ok(())
    }

    async fn delete(&self, key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        let uri = format!("{}/supervisors/{}", self.root, key).parse::<Uri>()?;
        let _ = self.inner.delete(uri).send().await?.bytes().await?;
        Ok(())
    }
}

use crate::client::GraphClient;
use crate::error::ApiClientError;
use async_trait::async_trait;
use toy_api::graph::{DeleteOption, FindOption, GraphEntity, GraphsEntity, ListOption, PutOption};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug)]
pub struct HttpGraphClient<T> {
    root: String,
    inner: T,
}

impl<T> HttpGraphClient<T>
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
impl<T> GraphClient for HttpGraphClient<T>
where
    T: HttpClient,
{
    async fn list(&self, _opt: ListOption) -> Result<GraphsEntity, ApiClientError> {
        let uri = format!("{}/graphs", self.root).parse::<Uri>()?;
        let bytes = self.inner.get(uri).send().await?.bytes().await?;
        let r = toy_pack_json::unpack::<GraphsEntity>(&bytes)?;
        Ok(r)
    }

    async fn find(
        &self,
        key: String,
        _opt: FindOption,
    ) -> Result<Option<GraphEntity>, ApiClientError> {
        let uri = format!("{}/graphs/{}", self.root, key).parse::<Uri>()?;
        let bytes = self.inner.get(uri).send().await?.bytes().await?;
        let r = toy_pack_json::unpack::<Option<GraphEntity>>(&bytes)?;
        Ok(r)
    }

    async fn put(
        &self,
        key: String,
        v: GraphEntity,
        _opt: PutOption,
    ) -> Result<(), ApiClientError> {
        let uri = format!("{}/graphs/{}", self.root, key).parse::<Uri>()?;
        let body = toy_pack_json::pack(&v)?;
        let _ = self.inner.put(uri).body(body).send().await?.bytes().await?;
        Ok(())
    }

    async fn delete(&self, key: String, _opt: DeleteOption) -> Result<(), ApiClientError> {
        let uri = format!("{}/graphs/{}", self.root, key).parse::<Uri>()?;
        let _ = self.inner.delete(uri).send().await?.bytes().await?;
        Ok(())
    }
}

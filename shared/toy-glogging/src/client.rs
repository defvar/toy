use crate::error::GLoggingError;
use crate::models::{
    ErrorResponse, ListRequest, ListResponse, TailRequest, TailResponse, WriteRequest,
    WriteResponse,
};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Body, IntoUrl};
use toy_gauth::GToken;
use toy_pack::deser::DeserializableOwned;

#[derive(Clone, Debug)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn from(c: reqwest::Client) -> Client {
        Client { inner: c }
    }

    /// clone raw client.
    pub fn raw(&self) -> reqwest::Client {
        self.inner.clone()
    }

    pub async fn list(
        &self,
        token: GToken,
        req: ListRequest,
    ) -> Result<ListResponse, GLoggingError> {
        tracing::debug!(req= ?req, "list");

        let param = toy_pack_json::pack(&req).unwrap();
        let res = self
            .request(
                "https://logging.googleapis.com/v2/entries:list",
                token,
                param,
            )
            .await?;
        self.to_response::<ListResponse, ErrorResponse>(res).await
    }

    pub async fn write(
        &self,
        token: GToken,
        req: WriteRequest,
    ) -> Result<WriteResponse, GLoggingError> {
        tracing::debug!(req= ?req, "write");

        let param = toy_pack_json::pack(&req).unwrap();
        let res = self
            .request(
                "https://logging.googleapis.com/v2/entries:write",
                token,
                param,
            )
            .await?;
        tracing::debug!("{:?}", res);
        self.to_response::<WriteResponse, ErrorResponse>(res).await
    }

    pub async fn tail(
        &self,
        token: GToken,
        req: TailRequest,
    ) -> Result<TailResponse, GLoggingError> {
        tracing::debug!(req= ?req, "tail");

        let param = toy_pack_json::pack(&req).unwrap();
        let res = self
            .request(
                "https://logging.googleapis.com/v2/entries:tail",
                token,
                param,
            )
            .await?;
        self.to_response::<TailResponse, Vec<ErrorResponse>>(res)
            .await
    }

    async fn to_response<T, E>(&self, res: reqwest::Response) -> Result<T, GLoggingError>
    where
        T: DeserializableOwned,
        E: DeserializableOwned + Into<GLoggingError>,
    {
        if res.status().is_success() {
            let bytes = res.bytes().await?;
            tracing::debug!("{:?}", std::str::from_utf8(&bytes));
            Ok(toy_pack_json::unpack::<T>(&bytes)?)
        } else {
            let bytes = res.bytes().await?;
            tracing::debug!("{:?}", std::str::from_utf8(&bytes));
            let e = toy_pack_json::unpack::<E>(&bytes)?;
            Err(e.into())
        }
    }

    async fn request<T: IntoUrl, B: Into<Body>>(
        &self,
        url: T,
        token: GToken,
        body: B,
    ) -> Result<reqwest::Response, GLoggingError> {
        let auth = HeaderValue::from_str(&format!("Bearer {}", token.access_token()))
            .map_err(|e| GLoggingError::error(e))?;
        self.inner
            .post(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(AUTHORIZATION, auth)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

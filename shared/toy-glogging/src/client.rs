use crate::error::GLoggingError;
use crate::models::{
    ErrorResponse, ListRequest, ListResponse, TailRequest, TailResponse, WriteRequest,
    WriteResponse,
};
use toy_gauth::GToken;
use toy_h::{
    header::HeaderValue, header::AUTHORIZATION, header::CONTENT_TYPE, Bytes, HttpClient,
    RequestBuilder, Response, StatusCode, Uri,
};
use toy_pack::deser::DeserializableOwned;

#[derive(Clone, Debug)]
pub struct Client<T> {
    inner: T,
}

impl<T> Client<T>
where
    T: HttpClient,
{
    pub fn from(c: T) -> Client<T> {
        Client { inner: c }
    }

    /// clone raw client.
    pub fn raw(&self) -> T {
        self.inner.clone()
    }

    pub async fn list(
        &self,
        token: GToken,
        req: ListRequest,
    ) -> Result<ListResponse, GLoggingError> {
        tracing::debug!(req= ?req, "list");

        let param = toy_pack_json::pack(&req).unwrap();
        let (status, bytes) = self
            .request(
                Uri::from_static("https://logging.googleapis.com/v2/entries:list"),
                token,
                param,
            )
            .await?;
        self.to_response::<ListResponse, ErrorResponse>(status, bytes)
            .await
    }

    pub async fn write(
        &self,
        token: GToken,
        req: WriteRequest,
    ) -> Result<WriteResponse, GLoggingError> {
        tracing::debug!("write");

        let param = toy_pack_json::pack(&req).unwrap();
        let (status, bytes) = self
            .request(
                Uri::from_static("https://logging.googleapis.com/v2/entries:write"),
                token,
                param,
            )
            .await?;
        self.to_response::<WriteResponse, ErrorResponse>(status, bytes)
            .await
    }

    pub async fn tail(
        &self,
        token: GToken,
        req: TailRequest,
    ) -> Result<TailResponse, GLoggingError> {
        tracing::debug!(req= ?req, "tail");

        let param = toy_pack_json::pack(&req).unwrap();
        let (status, bytes) = self
            .request(
                Uri::from_static("https://logging.googleapis.com/v2/entries:tail"),
                token,
                param,
            )
            .await?;
        self.to_response::<TailResponse, Vec<ErrorResponse>>(status, bytes)
            .await
    }

    async fn to_response<R, E>(&self, status: StatusCode, bytes: Bytes) -> Result<R, GLoggingError>
    where
        R: DeserializableOwned,
        E: DeserializableOwned + Into<GLoggingError>,
    {
        if status.is_success() {
            Ok(toy_pack_json::unpack::<R>(&bytes)?)
        } else {
            let e = toy_pack_json::unpack::<E>(&bytes)?;
            Err(e.into())
        }
    }

    async fn request<U: Into<Uri>, B: Into<Bytes>>(
        &self,
        url: U,
        token: GToken,
        body: B,
    ) -> Result<(StatusCode, Bytes), GLoggingError> {
        let auth = HeaderValue::from_str(&format!("Bearer {}", token.access_token()))
            .map_err(|e| GLoggingError::error(e))?;
        let res = self
            .inner
            .post(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(AUTHORIZATION, auth)
            .body(body)
            .send()
            .await?;

        let status = res.status();
        res.bytes().await.map(|x| (status, x)).map_err(|e| e.into())
    }
}

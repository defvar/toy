use crate::error::InfluxDBError;
use crate::models::flux_table::FluxTable;
use crate::models::line_protocol::{LineProtocolRecord, ToLineProtocol};
use crate::models::query_param::QueryParam;
use crate::models::ErrorInfo;
use crate::query::decode;
use toy_h::bytes::Buf;
use toy_h::{HttpClient, InvalidUri, RequestBuilder, Response, Uri};

#[derive(Clone, Debug)]
pub struct Client<T> {
    inner: T,
    root: Uri,
}

impl<T> Client<T>
where
    T: HttpClient,
{
    pub fn from<U>(c: T, uri: U) -> Result<Client<T>, InfluxDBError>
    where
        Uri: TryFrom<U, Error = InvalidUri>,
    {
        let uri = TryFrom::try_from(uri)?;
        Ok(Client {
            inner: c,
            root: uri,
        })
    }

    /// clone raw client.
    pub fn raw(&self) -> T {
        self.inner.clone()
    }

    pub async fn write<'a>(
        &self,
        token: &str,
        bucket: &str,
        org: &str,
        records: Vec<LineProtocolRecord<'a>>,
    ) -> Result<(), InfluxDBError> {
        let uri = Uri::try_from(&format!(
            "{}{}?bucket={}&org={}&precision={}",
            self.root, "api/v2/write", bucket, org, "ns"
        ))?;

        tracing::debug!("uri: {}", uri);

        let body = records.iter().try_fold(Vec::new(), |mut acc, x| {
            x.to_lp(&mut acc)?;
            acc.extend_from_slice(&b"\n"[..]);
            Ok::<Vec<u8>, InfluxDBError>(acc)
        })?;

        let res = self
            .inner
            .post(uri)
            .header(toy_h::header::AUTHORIZATION, format!("Token {}", token))
            .header(toy_h::header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .header(toy_h::header::ACCEPT, "application/json")
            .body(body)
            .send()
            .await?;
        if res.status().is_success() {
            let _ = res.bytes().await?;
            Ok(())
        } else {
            let bytes = res.bytes().await?;
            let e = toy_pack_json::unpack::<ErrorInfo>(&bytes)?;
            Err(InfluxDBError::api_error(e))
        }
    }

    pub async fn query(
        &self,
        token: &str,
        org: &str,
        param: &QueryParam,
    ) -> Result<Vec<FluxTable>, InfluxDBError> {
        let uri = Uri::try_from(&format!("{}{}?org={}", self.root, "api/v2/query", org))?;

        let json = toy_pack_json::pack_to_string(param)?;
        tracing::debug!("uri: {}", uri);
        tracing::debug!("influx query param: {}", json);

        let res = self
            .inner
            .post(uri)
            .header(toy_h::header::AUTHORIZATION, format!("Token {}", token))
            .header(toy_h::header::CONTENT_TYPE, "application/json")
            .header(toy_h::header::ACCEPT, "application/csv")
            .body(json)
            .send()
            .await?;
        if res.status().is_success() {
            let bytes = res.bytes().await?;
            decode(bytes.reader())
        } else {
            let bytes = res.bytes().await?;
            let e = toy_pack_json::unpack::<ErrorInfo>(&bytes)?;
            Err(InfluxDBError::api_error(e))
        }
    }
}

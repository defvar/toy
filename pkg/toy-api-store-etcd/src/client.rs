use crate::kv::*;
use crate::txn::{Compare, CompareResult, CompareTarget, RequestOp, TxnRequest, TxnResponse};
use crate::watch::{WatchCreateRequest, WatchResponse};
use futures_util::StreamExt;
use std::convert::TryFrom;
use toy_api_server::store::error::StoreError;
use toy_h::error::HError;
use toy_h::{
    header::HeaderValue, header::CONTENT_TYPE, ByteStream, HttpClient, InvalidUri, RequestBuilder,
    Response, Uri,
};

/// etcd api client.
#[derive(Clone, Debug)]
pub struct Client<T> {
    inner: T,
    root: Uri,
}

impl<T> Client<T>
where
    T: HttpClient,
{
    /// Create new client.
    /// Url is ectd api endpoint address.
    pub fn new<U>(client: T, uri: U) -> Result<Client<T>, StoreError>
    where
        Uri: TryFrom<U, Error = InvalidUri>,
    {
        let uri = TryFrom::try_from(uri)?;
        Ok(Client {
            inner: client,
            root: uri,
        })
    }

    pub async fn get<K>(&self, key: K) -> Result<SingleResponse, StoreError>
    where
        K: AsRef<str>,
    {
        let param = toy_pack_json::pack(&RangeRequest::single(key.as_ref())).unwrap();
        let bytes = self.range(param).await?;
        let res = toy_pack_json::unpack::<RangeResponse>(&bytes)?;
        tracing::debug!(key= ?key.as_ref(), "get");
        res.into_single()
    }

    pub async fn list<K>(&self, prefix: K) -> Result<RangeResponse, StoreError>
    where
        K: AsRef<str>,
    {
        let param = toy_pack_json::pack(&RangeRequest::range_from(prefix.as_ref())).unwrap();
        let bytes = self.range(param).await?;
        let res = toy_pack_json::unpack::<RangeResponse>(&bytes)?;
        tracing::debug!(prefix= ?prefix.as_ref(), "list");
        Ok(res)
    }

    pub async fn watch<K>(
        &self,
        prefix: K,
    ) -> Result<impl toy_h::Stream<Item = Result<WatchResponse, StoreError>>, StoreError>
    where
        K: AsRef<str>,
    {
        tracing::debug!(prefix= ?prefix.as_ref(), "watch start");
        let param = toy_pack_json::pack(&WatchCreateRequest::range_from(prefix.as_ref())).unwrap();
        self.watch0(param).await.map(|byte_stream| {
            byte_stream.map(|x| match x {
                Ok(v) => toy_pack_json::unpack::<WatchResponse>(&v).map_err(|e| e.into()),
                Err(e) => Err(e.into()),
            })
        })
    }

    pub async fn create<K, V>(&self, key: K, value: V) -> Result<TxnResponse, StoreError>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let txn = TxnRequest::with(
            Compare::not_exists(key.as_ref()),
            RequestOp::put(PutRequest::from(key.as_ref(), value.as_ref())),
        );
        let param = toy_pack_json::pack(&txn).unwrap();
        let bytes = self.txn(param).await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        tracing::debug!(key= ?key.as_ref(), "create");
        Ok(res)
    }

    pub async fn update<K, V>(
        &self,
        key: K,
        value: V,
        version: u64,
    ) -> Result<TxnResponse, StoreError>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let txn = TxnRequest::with(
            Compare::with(
                key.as_ref(),
                CompareResult::EQUAL,
                CompareTarget::MOD,
                version.to_string(),
            ),
            RequestOp::put(PutRequest::from(key.as_ref(), value.as_ref())),
        );
        let param = toy_pack_json::pack(&txn).unwrap();
        let bytes = self.txn(param).await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        tracing::debug!(key= ?key.as_ref(), version= ?version, "update");
        Ok(res)
    }

    pub async fn remove<K>(&self, key: K, version: u64) -> Result<TxnResponse, StoreError>
    where
        K: AsRef<str>,
    {
        let txn = TxnRequest::with(
            Compare::with(
                key.as_ref(),
                CompareResult::EQUAL,
                CompareTarget::MOD,
                version.to_string(),
            ),
            RequestOp::delete(DeleteRangeRequest::single(key.as_ref())),
        );
        let param = toy_pack_json::pack(&txn).unwrap();
        let bytes = self.txn(param).await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        tracing::debug!(key= ?key.as_ref(), "remove");
        Ok(res)
    }

    async fn txn(&self, body: Vec<u8>) -> Result<toy_h::bytes::Bytes, HError> {
        let uri = format!("{}v3/kv/txn", self.root).parse::<Uri>()?;
        self.inner
            .post(uri)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await?
            .bytes()
            .await
    }

    async fn range(&self, body: Vec<u8>) -> Result<toy_h::bytes::Bytes, HError> {
        let uri = format!("{}v3/kv/range", self.root).parse::<Uri>()?;
        self.inner
            .post(uri)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await?
            .bytes()
            .await
    }

    async fn watch0(&self, body: Vec<u8>) -> Result<ByteStream, StoreError> {
        let uri = format!("{}v3/watch", self.root).parse::<Uri>()?;
        Ok(self
            .inner
            .post(uri)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await?
            .stream())
    }
}

use crate::error::StoreEtcdError;
use crate::kv::*;
use crate::txn::{Compare, CompareResult, CompareTarget, RequestOp, TxnRequest, TxnResponse};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::{IntoUrl, Url};

/// etcd api client.
#[derive(Clone, Debug)]
pub struct Client {
    inner: reqwest::Client,
    root: Url,
}

impl Client {
    /// Create new client.
    /// Url is ectd api endpoint address.
    pub fn new(url: impl IntoUrl) -> Result<Client, StoreEtcdError> {
        let c = reqwest::Client::builder().build()?;
        let root = url.into_url()?;
        Ok(Client { inner: c, root })
    }

    pub async fn get<K>(&self, key: K) -> Result<SingleResponse, StoreEtcdError>
    where
        K: AsRef<str>,
    {
        log::trace!("get key={:?}", key.as_ref());
        let param = toy_pack_json::pack(&RangeRequest::single(key.as_ref())).unwrap();
        let bytes = self.range(param).await?.bytes().await?;
        let res = toy_pack_json::unpack::<RangeResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        res.into_single()
    }

    pub async fn list<K>(&self, key: K) -> Result<RangeResponse, StoreEtcdError>
    where
        K: AsRef<str>,
    {
        log::trace!("list key={:?}", key.as_ref());
        let param = toy_pack_json::pack(&RangeRequest::range_from(key.as_ref())).unwrap();
        let bytes = self.range(param).await?.bytes().await?;
        let res = toy_pack_json::unpack::<RangeResponse>(&bytes)?;
        log::trace!("list response={:?}", res);
        Ok(res)
    }

    pub async fn create<K, V>(&self, key: K, value: V) -> Result<TxnResponse, StoreEtcdError>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        log::trace!("create key={:?}", key.as_ref());
        let txn = TxnRequest::with(
            Compare::not_exists(key.as_ref()),
            RequestOp::put(PutRequest::from(key.as_ref(), value.as_ref())),
        );
        let param = toy_pack_json::pack(&txn).unwrap();
        let bytes = self.txn(param).await?.bytes().await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        Ok(res)
    }

    pub async fn update<K, V>(
        &self,
        key: K,
        value: V,
        version: u64,
    ) -> Result<TxnResponse, StoreEtcdError>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        log::trace!("update key={:?}, version={:?}", key.as_ref(), version);
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
        let bytes = self.txn(param).await?.bytes().await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        Ok(res)
    }

    pub async fn remove<K>(&self, key: K, version: u64) -> Result<TxnResponse, StoreEtcdError>
    where
        K: AsRef<str>,
    {
        log::trace!("remove key={:?}", key.as_ref());
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
        let bytes = self.txn(param).await?.bytes().await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        Ok(res)
    }

    async fn txn(&self, body: Vec<u8>) -> Result<reqwest::Response, reqwest::Error> {
        self.inner
            .post(format!("{}v3/kv/txn", self.root).as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await
    }

    async fn range(&self, body: Vec<u8>) -> Result<reqwest::Response, reqwest::Error> {
        self.inner
            .post(format!("{}v3/kv/range", self.root).as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await
    }
}

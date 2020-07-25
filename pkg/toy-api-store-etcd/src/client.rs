use crate::error::StoreEtcdError;
use crate::kv::*;
use crate::txn::{Compare, CompareResult, CompareTarget, RequestOp, TxnRequest, TxnResponse};
use reqwest::header::{HeaderValue, CONTENT_TYPE};

pub struct Client {
    inner: reqwest::Client,
    root: String,
}

impl Client {
    pub fn new<P: AsRef<str>>(root: P) -> Result<Client, StoreEtcdError> {
        let c = reqwest::Client::builder().build()?;
        Ok(Client {
            inner: c,
            root: root.as_ref().to_string(),
        })
    }

    pub async fn get<K>(&self, key: K) -> Result<RangeResponse, StoreEtcdError>
    where
        K: AsRef<str>,
    {
        log::trace!("get key={:?}", key.as_ref());
        let encoded_key = base64::encode(key.as_ref().as_bytes());
        let param = toy_pack_json::pack(&RangeRequest::single(&encoded_key)).unwrap();

        let bytes = self
            .inner
            .post(format!("{}/v3/kv/range", self.root).as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(param)
            .send()
            .await?
            .bytes()
            .await?;

        let res = toy_pack_json::unpack::<RangeResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        Ok(res)
    }

    pub async fn create<K, V>(&self, key: K, value: V) -> Result<TxnResponse, StoreEtcdError>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        log::trace!("create key={:?}", key.as_ref());
        let encoded_key = base64::encode(key.as_ref().as_bytes());
        let encoded_value = base64::encode(value.as_ref().as_bytes());
        let txn = TxnRequest::with(
            Compare::not_exists(&encoded_key),
            RequestOp::put(PutRequest::from(&encoded_key, &encoded_value)),
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
        let encoded_key = base64::encode(key.as_ref().as_bytes());
        let txn = TxnRequest::with(
            Compare::with(
                &encoded_key,
                CompareResult::EQUAL,
                CompareTarget::MOD,
                version.to_string(),
            ),
            RequestOp::delete(DeleteRangeRequest::single(&encoded_key)),
        );
        let param = toy_pack_json::pack(&txn).unwrap();
        let bytes = self.txn(param).await?.bytes().await?;
        let res = toy_pack_json::unpack::<TxnResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        Ok(res)
    }

    async fn txn(&self, body: Vec<u8>) -> Result<reqwest::Response, reqwest::Error> {
        self.inner
            .post(format!("{}/v3/kv/txn", self.root).as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .send()
            .await
    }
}

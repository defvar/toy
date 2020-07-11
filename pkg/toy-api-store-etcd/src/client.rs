use crate::error::StoreEtcdError;
use crate::types::*;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use toy_pack::deser::DeserializableOwned;

pub struct Client {
    inner: reqwest::Client,
    root: String,
}

impl Client {
    //TODO: error handle
    pub fn new<P: AsRef<str>>(root: P) -> Client {
        let c = reqwest::Client::builder().build().unwrap();
        Client {
            inner: c,
            root: root.as_ref().to_string(),
        }
    }

    pub async fn get<K>(&self, key: K) -> Result<KvRangeResponse, StoreEtcdError>
    where
        K: AsRef<str>,
        // V: DeserializableOwned,
    {
        log::trace!("get key={:?}", key.as_ref());
        let encoded_key = base64::encode(key.as_ref().as_bytes());
        let param = toy_pack_json::pack(&KvRangeRequest {
            key: encoded_key,
            range_end: None,
        })
        .unwrap();

        let bytes = self
            .inner
            .post(format!("{}/v3/kv/range", self.root).as_str())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(param)
            .send()
            .await?
            .bytes()
            .await?;

        let res = toy_pack_json::unpack::<KvRangeResponse>(&bytes)?;
        log::trace!("get response={:?}", res);
        Ok(res)
    }
}

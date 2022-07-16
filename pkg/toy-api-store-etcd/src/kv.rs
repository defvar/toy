use crate::error::EtcdError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use toy_api_server::store::error::StoreError;

#[derive(Debug)]
pub struct Versioning {
    key: Vec<u8>,
    data: Vec<u8>,
    version: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct ResponseHeader {
    cluster_id: Option<String>,
    member_id: Option<String>,
    revision: String,
    raft_term: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Kv {
    key: String,
    create_revision: Option<String>,
    mod_revision: String,
    version: Option<String>,
    value: Option<String>,
}

///////////////////////////////
// Range
///////////////////////////////

#[derive(Debug, Serialize)]
pub struct RangeRequest {
    key: String,
    range_end: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RangeResponse {
    header: ResponseHeader,
    #[serde(default)]
    kvs: Vec<Kv>,
    count: Option<String>,
}

/// convert only from `RangeResponse'
#[allow(dead_code)]
#[derive(Debug)]
pub struct SingleResponse {
    header: ResponseHeader,
    kv: Option<Kv>,
    count: Option<String>,
}

///////////////////////////////
// Put
///////////////////////////////

#[derive(Debug, Serialize)]
pub struct PutRequest {
    key: String,
    value: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PutResponse {
    header: ResponseHeader,
}

///////////////////////////////
// Delete
///////////////////////////////

#[derive(Debug, Serialize)]
pub struct DeleteRangeRequest {
    key: String,
    range_end: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DeleteRangeResponse {
    header: ResponseHeader,
    deleted: String,
    #[serde(default)]
    prev_kvs: Vec<Kv>,
}

///////////////////////////////
// Impl
///////////////////////////////

impl Versioning {
    pub fn from(key: Vec<u8>, data: Vec<u8>, version: u64) -> Versioning {
        Versioning { key, data, version }
    }

    pub fn key(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.key) }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }

    pub fn unpack<T, F>(self, f: F) -> Result<T, StoreError>
    where
        T: DeserializeOwned,
        F: FnOnce(Self) -> Result<T, StoreError>,
    {
        f(self)
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}

impl Kv {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|x| x.as_str())
    }

    pub fn mod_revision(&self) -> &str {
        &self.mod_revision
    }

    pub fn to_versioning(&self) -> Result<Versioning, EtcdError> {
        let key = decode(&self.key);
        let val = if self.value.is_some() {
            decode(self.value().unwrap())
        } else {
            Ok(Vec::with_capacity(0))
        };

        match (key, val) {
            (Ok(k), Ok(v)) => {
                let version = self.mod_revision.parse::<u64>()?;
                Ok(Versioning::from(k, v, version))
            }
            (Err(e), _) | (_, Err(e)) => Err(e.into()),
        }
    }
}

impl RangeRequest {
    pub fn single(key: &str) -> RangeRequest {
        let encoded_key = encode(key);
        RangeRequest {
            key: encoded_key,
            range_end: None,
        }
    }

    pub fn range_from(key: &str) -> RangeRequest {
        let encoded_key = encode(key);
        let range_end = {
            std::str::from_utf8(get_range_end(key).as_slice())
                .map(|x| encode(x.to_string()))
                .ok()
        };
        RangeRequest {
            key: encoded_key,
            range_end,
        }
    }

    pub fn all() -> RangeRequest {
        RangeRequest {
            key: "AA==".to_string(),
            range_end: None,
        }
    }
}

impl RangeResponse {
    pub fn values(&self) -> Result<Vec<Versioning>, StoreError> {
        self.kvs.iter().try_fold(Vec::new(), |mut vec, x| {
            let v = x.to_versioning()?;
            vec.push(v);
            Ok(vec)
        })
    }

    pub fn unpack<T, F>(&self, mut f: F) -> Result<Vec<T>, StoreError>
    where
        T: DeserializeOwned,
        F: FnMut(Versioning) -> Result<T, StoreError>,
    {
        self.kvs.iter().try_fold(Vec::new(), |mut vec, x| {
            let v = x.to_versioning()?;
            let v = f(v)?;
            vec.push(v);
            Ok(vec)
        })
    }

    pub fn into_single(mut self) -> Result<SingleResponse, StoreError> {
        if self.kvs.len() == 0 {
            Ok(SingleResponse {
                header: self.header,
                kv: None,
                count: Some("0".to_string()),
            })
        } else if self.kvs.len() == 1 {
            Ok(SingleResponse {
                header: self.header,
                kv: Some(self.kvs.pop().unwrap()),
                count: Some("1".to_string()),
            })
        } else {
            let one = self.kvs.pop().unwrap();
            Err(StoreError::multiple_result(&one.key))
        }
    }
}

impl SingleResponse {
    pub fn value(&self) -> Result<Option<Versioning>, EtcdError> {
        match &self.kv {
            Some(kv) => kv.to_versioning().map(Some),
            None => Ok(None),
        }
    }
}

impl PutRequest {
    pub fn from(key: &str, value: &str) -> PutRequest {
        let encoded_key = encode(key);
        let encoded_value = encode(value);
        PutRequest {
            key: encoded_key,
            value: encoded_value,
        }
    }
}

impl PutResponse {
    pub fn revision(&self) -> Result<u64, EtcdError> {
        self.header.revision.parse::<u64>().map_err(|e| e.into())
    }
}

impl DeleteRangeRequest {
    pub fn single(key: &str) -> DeleteRangeRequest {
        let encoded_key = encode(key);
        DeleteRangeRequest {
            key: encoded_key,
            range_end: None,
        }
    }
}

pub(crate) fn encode<T: AsRef<[u8]>>(input: T) -> String {
    base64::encode_config(input.as_ref(), base64::URL_SAFE)
}

pub(crate) fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(input.as_ref(), base64::URL_SAFE)
}

pub(crate) fn get_range_end(key: &str) -> Vec<u8> {
    let mut end = key.clone().as_bytes().to_vec();
    for i in (0..end.len()).rev() {
        if end[i] < 0xff {
            end[i] += 1;
            end = end[0..=i].to_vec();
            break;
        }
    }
    end
}

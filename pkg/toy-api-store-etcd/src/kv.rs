use crate::error::StoreEtcdError;
use toy_pack::deser::DeserializableOwned;
use toy_pack::{Pack, Unpack};

#[derive(Debug)]
pub struct Versioning<T> {
    data: T,
    version: u64,
}

#[derive(Debug, Unpack, Default)]
pub struct ResponseHeader {
    cluster_id: String,
    member_id: String,
    revision: String,
    raft_term: String,
}

#[derive(Debug, Pack, Unpack)]
pub struct Kv {
    key: String,
    create_revision: String,
    mod_revision: String,
    version: String,
    value: String,
}

///////////////////////////////
// Range
///////////////////////////////

#[derive(Debug, Pack)]
#[toy(ignore_pack_if_none)]
pub struct RangeRequest {
    key: String,
    range_end: Option<String>,
}

#[derive(Debug, Unpack)]
pub struct RangeResponse {
    header: ResponseHeader,
    kvs: Vec<Kv>,
    count: String,
}

/// convert only from `RangeResponse'
#[derive(Debug)]
pub struct SingleResponse {
    header: ResponseHeader,
    kv: Option<Kv>,
    count: String,
}

///////////////////////////////
// Put
///////////////////////////////

#[derive(Debug, Pack)]
pub struct PutRequest {
    key: String,
    value: String,
}

#[derive(Debug, Unpack)]
pub struct PutResponse {
    header: ResponseHeader,
}

///////////////////////////////
// Delete
///////////////////////////////

#[derive(Debug, Pack)]
pub struct DeleteRangeRequest {
    key: String,
    range_end: Option<String>,
}

#[derive(Debug, Unpack)]
pub struct DeleteRangeResponse {
    header: ResponseHeader,
    deleted: String,
    prev_kvs: Vec<Kv>,
}

///////////////////////////////
// Impl
///////////////////////////////

impl<T> Versioning<T> {
    pub fn from(data: T, version: u64) -> Versioning<T> {
        Versioning { data, version }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn version(&self) -> u64 {
        self.version
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
    pub fn json<T>(&self) -> Result<Vec<Versioning<T>>, StoreEtcdError>
    where
        T: DeserializableOwned,
    {
        self.kvs
            .iter()
            .try_fold(Vec::new(), |mut vec, x| match decode(&x.value) {
                Ok(v) => {
                    let r = toy_pack_json::unpack::<T>(&v)?;
                    let version = x.mod_revision.parse::<u64>()?;
                    vec.push(Versioning::<T>::from(r, version));
                    Ok(vec)
                }
                Err(e) => Err(e.into()),
            })
    }

    pub fn into_single(mut self) -> Result<SingleResponse, StoreEtcdError> {
        if self.kvs.len() == 0 {
            Ok(SingleResponse {
                header: self.header,
                kv: None,
                count: "0".to_string(),
            })
        } else if self.kvs.len() == 1 {
            Ok(SingleResponse {
                header: self.header,
                kv: Some(self.kvs.pop().unwrap()),
                count: "1".to_string(),
            })
        } else {
            let one = self.kvs.pop().unwrap();
            Err(StoreEtcdError::multiple_result(&one.key))
        }
    }
}

impl SingleResponse {
    pub fn json<T>(&self) -> Result<Option<Versioning<T>>, StoreEtcdError>
    where
        T: DeserializableOwned,
    {
        match &self.kv {
            Some(kv) => match decode(&kv.value) {
                Ok(v) => {
                    let r = toy_pack_json::unpack::<T>(&v)?;
                    let version = kv.mod_revision.parse::<u64>()?;
                    Ok(Some(Versioning::<T>::from(r, version)))
                }
                Err(e) => Err(e.into()),
            },
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

fn get_range_end(key: &str) -> Vec<u8> {
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

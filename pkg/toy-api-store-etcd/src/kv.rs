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

///////////////////////////////
// Put
///////////////////////////////

#[derive(Debug, Pack)]
pub struct PutRequest<'a> {
    key: &'a str,
    value: &'a str,
}

#[derive(Debug, Unpack)]
pub struct PutResponse {
    header: ResponseHeader,
}

///////////////////////////////
// Delete
///////////////////////////////

#[derive(Debug, Pack)]
pub struct DeleteRangeRequest<'a> {
    key: &'a str,
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
        RangeRequest {
            key: key.to_string(),
            range_end: None,
        }
    }

    pub fn range_from(key: &str) -> RangeRequest {
        let range_end = {
            std::str::from_utf8(get_range_end(key).as_slice())
                .map(|x| x.to_string())
                .ok()
        };
        RangeRequest {
            key: key.to_string(),
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
            .try_fold(Vec::new(), |mut vec, x| match base64::decode(&x.value) {
                Ok(v) => {
                    log::trace!("get decode(base64)={:?}", std::str::from_utf8(&v));
                    let r = toy_pack_json::unpack::<T>(&v)?;
                    let version = x.mod_revision.parse::<u64>()?;
                    vec.push(Versioning::<T>::from(r, version));
                    Ok(vec)
                }
                Err(e) => Err(e.into()),
            })
    }
}

impl<'a> PutRequest<'a> {
    pub fn from(key: &'a str, value: &'a str) -> PutRequest<'a> {
        PutRequest { key, value }
    }
}

impl<'a> DeleteRangeRequest<'a> {
    pub fn single(key: &'a str) -> DeleteRangeRequest<'a> {
        DeleteRangeRequest {
            key,
            range_end: None,
        }
    }
}

fn get_range_end(key: &str) -> Vec<u8> {
    let mut end = key.clone().as_bytes().to_vec();
    for i in (0..end.len()).rev() {
        if end[i] < 0xff {
            end[i] += 1;
            end = end[..=i].to_vec();
            break;
        }
    }
    end
}

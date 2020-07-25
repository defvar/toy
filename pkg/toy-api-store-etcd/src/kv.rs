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
                    vec.push(Versioning::<T> { data: r, version });
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

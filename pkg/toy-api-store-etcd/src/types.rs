use crate::error::StoreEtcdError;
use toy_pack::deser::DeserializableOwned;
use toy_pack::{Pack, Unpack};

#[derive(Pack)]
pub(crate) struct KvRangeRequest {
    pub(crate) key: String,
    pub(crate) range_end: Option<String>,
}

#[derive(Debug, Unpack)]
pub struct KvRangeResponse {
    pub(crate) kvs: Vec<Kv>,
    pub(crate) count: String,
}

#[derive(Debug, Unpack)]
pub(crate) struct Kv {
    pub(crate) key: String,
    pub(crate) create_revision: String,
    pub(crate) mod_revision: String,
    pub(crate) version: String,
    pub(crate) value: String,
}

pub(crate) struct KvPutRequest<T> {
    pub(crate) key: String,
    pub(crate) value: T,
}

impl KvRangeResponse {
    pub fn json<T>(&self) -> Result<Vec<T>, StoreEtcdError>
    where
        T: DeserializableOwned,
    {
        self.kvs
            .iter()
            .try_fold(Vec::new(), |mut vec, x| match base64::decode(&x.value) {
                Ok(v) => {
                    log::trace!("get decode(base64)={:?}", std::str::from_utf8(&v));
                    let r = toy_pack_json::unpack::<T>(&v)?;
                    vec.push(r);
                    Ok(vec)
                }
                Err(e) => Err(e.into()),
            })
    }
}

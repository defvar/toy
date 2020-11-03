use crate::store::error::StoreError;
use crate::store::store_op::*;
use crate::store::{StoreConnection, StoreOps, StoreOpsFactory};
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct BTreeStoreConnection {
    map: Arc<Mutex<BTreeMap<String, Vec<u8>>>>,
}

impl StoreConnection for BTreeStoreConnection {}

#[derive(Clone, Debug)]
pub struct BTreeStoreOps;

#[derive(Clone, Debug)]
pub struct BTreeStoreOpsFactory;

impl StoreOps<BTreeStoreConnection> for BTreeStoreOps {}

impl List for BTreeStoreOps {
    type Con = BTreeStoreConnection;
    type T = impl Future<Output = Result<Vec<Vec<u8>>, Self::Err>> + Send;
    type Err = StoreError;

    fn list(&self, con: Self::Con, prefix: String, opt: ListOption) -> Self::T {
        async move {
            let map = con.map.lock().unwrap();
            let vec: Vec<_> = map
                .iter()
                .filter(|(k, _)| k.starts_with(&prefix))
                .map(|(_, v)| v.clone())
                .collect();
            if vec.len() == 0 {
                log::debug!("[list] not found. prefix:{:?}, opt:{:?}", prefix, opt);
            }
            Ok(vec)
        }
    }
}

impl Find for BTreeStoreOps {
    type Con = BTreeStoreConnection;
    type T = impl Future<Output = Result<Option<Vec<u8>>, Self::Err>> + Send;
    type Err = StoreError;

    fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T {
        async move {
            let map = con.map.lock().unwrap();
            match map.get(&key) {
                Some(v) => Ok(Some(v.clone())),
                _ => {
                    log::debug!("[find] not found. key:{:?}, opt:{:?}", key, opt);
                    Ok(Option::<Vec<u8>>::None)
                }
            }
        }
    }
}

impl Put for BTreeStoreOps {
    type Con = BTreeStoreConnection;
    type T = impl Future<Output = Result<PutResult, Self::Err>> + Send;
    type Err = StoreError;

    fn put(&self, con: Self::Con, key: String, v: Vec<u8>, opt: PutOption) -> Self::T {
        async move {
            let mut map = con.map.lock().unwrap();
            if let Some(prev) = map.insert(key.clone(), v) {
                log::debug!(
                    "[put] update previous value. key:{:?}, opt:{:?}, prev:{:?}",
                    key,
                    opt,
                    prev
                );
                Ok(PutResult::Update)
            } else {
                Ok(PutResult::Create)
            }
        }
    }
}

impl Delete for BTreeStoreOps {
    type Con = BTreeStoreConnection;
    type T = impl Future<Output = Result<DeleteResult, Self::Err>> + Send;
    type Err = StoreError;

    fn delete(&self, con: Self::Con, key: String, opt: DeleteOption) -> Self::T {
        async move {
            let mut map = con.map.lock().unwrap();
            match map.remove(&key) {
                None => {
                    log::debug!("[delete] not found. key:{:?}, opt:{:?}", key, opt);
                    Ok(DeleteResult::NotFound)
                }
                _ => Ok(DeleteResult::Deleted),
            }
        }
    }
}

impl StoreOpsFactory<BTreeStoreConnection> for BTreeStoreOpsFactory {
    type Ops = BTreeStoreOps;

    fn create(&self) -> Result<Self::Ops, StoreError> {
        Ok(BTreeStoreOps)
    }

    fn connect(&self) -> Result<BTreeStoreConnection, StoreError> {
        Ok(BTreeStoreConnection {
            map: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }
}

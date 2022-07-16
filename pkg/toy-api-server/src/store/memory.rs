use crate::store::error::StoreError;
use crate::store::kv::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, KvResponse, KvStore, KvStoreOps,
    KvWatchResponse, List, ListOption, Put, PutOption, PutResult, Update, UpdateResult, Watch,
    WatchOption,
};
use crate::store::StoreConnection;
use futures_util::stream::BoxStream;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use toy_core::data::Value;
use toy_h::NoopHttpClient;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct MemoryStore {
    map: Arc<Mutex<HashMap<String, Value>>>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        MemoryStore {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_map(init: HashMap<String, Value>) -> MemoryStore {
        MemoryStore {
            map: Arc::new(Mutex::new(init)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStoreOps;

impl StoreConnection for MemoryStore {}

impl KvStoreOps<MemoryStore> for MemoryStoreOps {}

impl KvStore<NoopHttpClient> for MemoryStore {
    type Con = MemoryStore;
    type Ops = MemoryStoreOps;

    fn con(&self) -> Option<Self::Con> {
        Some(self.clone())
    }

    fn ops(&self) -> Self::Ops {
        MemoryStoreOps
    }

    fn establish(&mut self, _: NoopHttpClient) -> Result<(), StoreError> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl Find for MemoryStoreOps {
    type Con = MemoryStore;

    #[instrument(skip(self, con))]
    async fn find<V>(
        &self,
        con: Self::Con,
        key: String,
        opt: FindOption,
    ) -> Result<Option<KvResponse<V>>, StoreError>
    where
        V: DeserializeOwned,
    {
        tracing::debug!("find key:{:?}", key);
        let r = {
            let lock = con.map.lock().unwrap();
            lock.get(&key).map(|x| x.clone())
        };
        match r {
            Some(v) => {
                let v = toy_core::data::unpack::<V>(&v)?;
                Ok(Some(KvResponse::with_version(v, 0)))
            }
            None => {
                tracing::debug!("[find] not found. key:{:?}, opt:{:?}", key, opt);
                Ok(Option::<KvResponse<V>>::None)
            }
        }
    }
}

#[async_trait::async_trait]
impl List for MemoryStoreOps {
    type Con = MemoryStore;

    #[instrument(skip(self, con))]
    async fn list<V>(
        &self,
        con: Self::Con,
        prefix: String,
        opt: ListOption,
    ) -> Result<Vec<KvResponse<V>>, StoreError>
    where
        V: DeserializeOwned,
    {
        tracing::debug!("list prefix:{:?}, opt:{:?}", prefix, opt);
        let r = {
            let lock = con.map.lock().unwrap();
            lock.iter()
                .filter(|(k, _)| k.starts_with(&prefix))
                .try_fold(Vec::new(), |mut vec, (_, v)| {
                    let v = toy_core::data::unpack::<V>(&v)?;
                    vec.push(KvResponse::with_version(v, 0));
                    Ok(vec)
                })
        };
        r
    }
}

#[async_trait::async_trait]
impl Put for MemoryStoreOps {
    type Con = MemoryStore;

    #[instrument(skip(self, con, v))]
    async fn put<V>(
        &self,
        con: Self::Con,
        key: String,
        v: V,
        opt: PutOption,
    ) -> Result<PutResult, StoreError>
    where
        V: Serialize + Send,
    {
        tracing::debug!("put key:{:?}, opt:{:?}", key, opt);
        let v = toy_core::data::pack(&v)?;

        let mut lock = con.map.lock().unwrap();
        Ok(match lock.insert(key, v) {
            Some(_) => PutResult::Update(0),
            None => PutResult::Create,
        })
    }
}

#[async_trait::async_trait]
impl Delete for MemoryStoreOps {
    type Con = MemoryStore;

    #[instrument(skip(self, con))]
    async fn delete(
        &self,
        con: Self::Con,
        key: String,
        opt: DeleteOption,
    ) -> Result<DeleteResult, StoreError> {
        tracing::debug!("delete key:{:?}, opt:{:?}", key, opt);
        let mut lock = con.map.lock().unwrap();
        Ok(match lock.remove(&key) {
            Some(_) => DeleteResult::Deleted,
            None => DeleteResult::NotFound,
        })
    }
}

#[async_trait::async_trait]
impl Watch for MemoryStoreOps {
    type Con = MemoryStore;

    #[instrument(skip(self, _con))]
    async fn watch<V>(
        &self,
        _con: Self::Con,
        _prefix: String,
        _opt: WatchOption,
    ) -> Result<BoxStream<Result<KvWatchResponse<V>, StoreError>>, StoreError>
    where
        V: DeserializeOwned,
    {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl Update for MemoryStoreOps {
    type Con = MemoryStore;

    #[instrument(skip(self, con, f))]
    async fn update<V, F>(
        &self,
        con: Self::Con,
        key: String,
        f: F,
    ) -> Result<UpdateResult, StoreError>
    where
        V: DeserializeOwned + Serialize + Send,
        F: FnOnce(V) -> Option<V> + Send,
    {
        tracing::debug!("update key:{:?}", key);

        let mut lock = con.map.lock().unwrap();
        let r = lock.get(&key).map(|x| x.clone());

        match r {
            Some(v) => {
                let v = toy_core::data::unpack::<V>(&v)?;
                let v = f(v);
                if v.is_some() {
                    let v = toy_core::data::pack(&v)?;
                    lock.insert(key, v);
                    Ok(UpdateResult::Update(0))
                } else {
                    Ok(UpdateResult::None)
                }
            }
            None => {
                tracing::debug!("[update] not found. key:{:?}", key);
                Ok(UpdateResult::NotFound)
            }
        }
    }
}

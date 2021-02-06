// //! In Memory Store.(BTreeMap)
// //!
//
// use crate::graph::store::{
//     Delete, DeleteOption, DeleteResult, GraphStore, GraphStoreOps, List, ListOption, Put,
//     PutOption, PutResult,
// };
// use crate::store::error::StoreError;
// use crate::store::kv::{Find, FindOption};
// use crate::store::StoreConnection;
// use async_trait::async_trait;
// use std::collections::BTreeMap;
// use std::future::Future;
// use std::sync::{Arc, Mutex};
// use toy_api::graph::GraphEntity;
// use toy_h::NoopHttpClient;
// use toy_pack::deser::DeserializableOwned;
//
// #[derive(Clone, Debug)]
// pub struct BTreeStore {
//     con: Option<BTreeStoreConnection>,
// }
//
// #[derive(Clone, Debug)]
// pub struct BTreeStoreConnection {
//     map: Arc<Mutex<BTreeMap<String, dyn DeserializableOwned>>>,
// }
//
// #[derive(Clone, Debug)]
// pub struct BTreeStoreOps;
//
// impl BTreeStore {
//     pub fn new() -> BTreeStore {
//         Self { con: None }
//     }
// }
//
// impl StoreConnection for BTreeStoreConnection {}
//
// impl GraphStoreOps<BTreeStoreConnection> for BTreeStoreOps {}
//
// impl GraphStore<NoopHttpClient> for BTreeStore {
//     type Con = BTreeStoreConnection;
//     type Ops = BTreeStoreOps;
//
//     fn con(&self) -> Option<Self::Con> {
//         self.con.clone()
//     }
//
//     fn ops(&self) -> Self::Ops {
//         BTreeStoreOps
//     }
//
//     fn establish(&mut self, _client: NoopHttpClient) -> Result<(), StoreError> {
//         self.con = Some(BTreeStoreConnection {
//             map: Arc::new(Mutex::new(BTreeMap::new())),
//         });
//         Ok(())
//     }
// }
//
// impl List for BTreeStoreOps {
//     type Con = BTreeStoreConnection;
//     type T = impl Future<Output = Result<Vec<GraphEntity>, Self::Err>> + Send;
//     type Err = StoreError;
//
//     fn list(&self, con: Self::Con, prefix: String, opt: ListOption) -> Self::T {
//         async move {
//             let map = con.map.lock().unwrap();
//             let vec: Vec<_> = map
//                 .iter()
//                 .filter(|(k, _)| k.starts_with(&prefix))
//                 .map(|(_, v)| v.clone())
//                 .collect();
//             if vec.len() == 0 {
//                 log::debug!("[list] not found. prefix:{:?}, opt:{:?}", prefix, opt);
//             }
//             Ok(vec)
//         }
//     }
// }
//
// #[async_trait]
// impl Find for BTreeStoreOps {
//     type Con = BTreeStoreConnection;
//     type Err = StoreError;
//
//     async fn find<V>(
//         &self,
//         con: Self::Con,
//         key: String,
//         opt: FindOption,
//     ) -> Result<Option<V>, Self::Err>
//     where
//         V: DeserializableOwned,
//     {
//         let map = con.map.lock().unwrap();
//         match map.get(&key) {
//             Some(v) => Ok(Some(v.clone())),
//             _ => {
//                 log::debug!("[find] not found. key:{:?}, opt:{:?}", key, opt);
//                 Ok(Option::<V>::None)
//             }
//         }
//     }
// }
//
// impl Put for BTreeStoreOps {
//     type Con = BTreeStoreConnection;
//     type T = impl Future<Output = Result<PutResult, Self::Err>> + Send;
//     type Err = StoreError;
//
//     fn put(&self, con: Self::Con, key: String, v: GraphEntity, opt: PutOption) -> Self::T {
//         async move {
//             let mut map = con.map.lock().unwrap();
//             if let Some(prev) = map.insert(key.clone(), v) {
//                 log::debug!(
//                     "[put] update previous value. key:{:?}, opt:{:?}, prev:{:?}",
//                     key,
//                     opt,
//                     prev
//                 );
//                 Ok(PutResult::Update)
//             } else {
//                 Ok(PutResult::Create)
//             }
//         }
//     }
// }
//
// impl Delete for BTreeStoreOps {
//     type Con = BTreeStoreConnection;
//     type T = impl Future<Output = Result<DeleteResult, Self::Err>> + Send;
//     type Err = StoreError;
//
//     fn delete(&self, con: Self::Con, key: String, opt: DeleteOption) -> Self::T {
//         async move {
//             let mut map = con.map.lock().unwrap();
//             match map.remove(&key) {
//                 None => {
//                     log::debug!("[delete] not found. key:{:?}, opt:{:?}", key, opt);
//                     Ok(DeleteResult::NotFound)
//                 }
//                 _ => Ok(DeleteResult::Deleted),
//             }
//         }
//     }
// }

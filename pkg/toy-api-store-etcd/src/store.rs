use crate::error::StoreEtcdError;
use crate::Client;
use std::future::Future;
use toy_api_server::store::error::StoreError;
use toy_api_server::store::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult, StoreConnection, StoreOps, StoreOpsFactory, Value,
};
use tracing::{span, Instrument, Level};

#[derive(Clone, Debug)]
pub struct EtcdStoreConnection {
    client: Client,
}

#[derive(Clone, Debug)]
pub struct EtcdStoreOps;

#[derive(Clone, Debug)]
pub struct EtcdStoreOpsFactory;

impl StoreConnection for EtcdStoreConnection {}

impl StoreOps<EtcdStoreConnection> for EtcdStoreOps {}

impl Find for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<Option<Value>, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T {
        let span = span!(Level::DEBUG, "find", key = ?key);
        async move {
            let res = con.client.get(&key).await?.json::<Value>()?;
            match res {
                Some(v) => Ok(Some(v.into_data())),
                None => {
                    tracing::debug!("[find] not found. key:{:?}, opt:{:?}", key, opt);
                    Ok(Option::<Value>::None)
                }
            }
        }
        .instrument(span)
    }
}

impl List for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<Vec<Value>, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn list(&self, con: Self::Con, prefix: String, _opt: ListOption) -> Self::T {
        let span = span!(Level::DEBUG, "list", prefix = ?prefix);
        async move {
            let res = con.client.list(&prefix).await?.json::<Value>()?;
            Ok(res.into_iter().map(|x| x.into_data()).collect())
        }
        .instrument(span)
    }
}

impl Put for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<PutResult, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn put(&self, con: Self::Con, key: String, v: Value, _opt: PutOption) -> Self::T {
        let span = span!(Level::DEBUG, "put", key = ?key);
        async move {
            let data_json = toy_pack_json::pack_to_string(&v)?;
            let create_res = con.client.create(&key, &data_json).await?;
            if create_res.is_success() {
                Ok(PutResult::Create)
            } else {
                // update
                let res = con.client.get(&key).await?.json::<Value>()?;
                match res {
                    Some(v) => {
                        let version = v.version();
                        let upd_res = con.client.update(&key, &data_json, version).await?;
                        if upd_res.is_success() {
                            Ok(PutResult::Update)
                        } else {
                            Err(StoreEtcdError::failed_opration("update", &key))
                        }
                    }
                    None => Err(StoreEtcdError::not_found(&key)),
                }
            }
        }
        .instrument(span)
    }
}

impl Delete for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<DeleteResult, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn delete(&self, con: Self::Con, key: String, _opt: DeleteOption) -> Self::T {
        let span = span!(Level::DEBUG, "delete", key = ?key);
        async move {
            let single_res = con.client.get(&key).await?.json::<Value>()?;
            match single_res {
                Some(v) => {
                    let rm_res = con.client.remove(&key, v.version()).await?;
                    if !rm_res.is_success() {
                        Err(StoreEtcdError::failed_opration("delete", &key))
                    } else {
                        Ok(DeleteResult::Deleted)
                    }
                }
                None => Ok(DeleteResult::NotFound),
            }
        }
        .instrument(span)
    }
}

impl StoreOpsFactory<EtcdStoreConnection> for EtcdStoreOpsFactory {
    type Ops = EtcdStoreOps;

    fn create(&self) -> Result<Self::Ops, StoreError> {
        Ok(EtcdStoreOps)
    }

    fn connect(&self) -> Result<EtcdStoreConnection, StoreError> {
        let host = std::env::var("TOY_STORE_ETCD_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("TOY_STORE_ETCD_PORT").unwrap_or_else(|_| "2379".to_string());
        let url = format!("http://{}:{}", host, port);
        tracing::info!("toy store=etcd. connecting:{}", &url);
        let c = match Client::new(&url) {
            Ok(c) => c,
            Err(e) => return Err(StoreError::error(e)),
        };
        Ok(EtcdStoreConnection { client: c })
    }
}

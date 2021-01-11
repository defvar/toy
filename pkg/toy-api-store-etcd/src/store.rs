use crate::error::StoreEtcdError;
use crate::Client;
use std::future::Future;
use toy_api_server::graph::models::GraphEntity;
use toy_api_server::graph::store::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, GraphStoreOps, List, ListOption, Put,
    PutOption, PutResult,
};
use toy_api_server::store::{error::StoreError, StoreConnection, StoreOpsFactory};
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

impl GraphStoreOps<EtcdStoreConnection> for EtcdStoreOps {}

impl Find for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<Option<GraphEntity>, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T {
        let span = span!(Level::DEBUG, "find", key = ?key);
        async move {
            let res = con.client.get(&key).await?.value()?;
            match res {
                Some(v) => {
                    let r = toy_pack_json::unpack(&v.into_data())?;
                    Ok(Some(r))
                }
                None => {
                    tracing::debug!("[find] not found. key:{:?}, opt:{:?}", key, opt);
                    Ok(Option::<GraphEntity>::None)
                }
            }
        }
        .instrument(span)
    }
}

impl List for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<Vec<GraphEntity>, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn list(&self, con: Self::Con, prefix: String, _opt: ListOption) -> Self::T {
        let span = span!(Level::DEBUG, "list", prefix = ?prefix);
        async move {
            let res = con.client.list(&prefix).await?.values()?;

            let r = res.into_iter().try_fold(Vec::new(), |mut vec, x| {
                let r = toy_pack_json::unpack::<GraphEntity>(&x.into_data());
                match r {
                    Ok(entity) => {
                        vec.push(entity);
                        Ok(vec)
                    }
                    Err(e) => Err(e),
                }
            })?;

            Ok(r)
        }
        .instrument(span)
    }
}

impl Put for EtcdStoreOps {
    type Con = EtcdStoreConnection;
    type T = impl Future<Output = Result<PutResult, Self::Err>> + Send;
    type Err = StoreEtcdError;

    fn put(&self, con: Self::Con, key: String, v: GraphEntity, _opt: PutOption) -> Self::T {
        let span = span!(Level::DEBUG, "put", key = ?key);
        async move {
            let s = toy_pack_json::pack_to_string(&v)?;
            let create_res = con.client.create(&key, &s).await?;
            if create_res.is_success() {
                Ok(PutResult::Create)
            } else {
                // update
                let res = con.client.get(&key).await?.value()?;
                match res {
                    Some(v) => {
                        let version = v.version();
                        let upd_res = con.client.update(&key, &s, version).await?;
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
            let single_res = con.client.get(&key).await?.value()?;
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

impl StoreOpsFactory for EtcdStoreOpsFactory {
    type Con = EtcdStoreConnection;
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

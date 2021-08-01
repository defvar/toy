use crate::watch::{EventType, WatchResponse};
use crate::Client;
use futures_util::stream::BoxStream;
use futures_util::StreamExt;
use std::future::Future;
use std::marker::PhantomData;
use toy_api::task::PendingTask;
use toy_api_server::store::kv::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, KvResponse, KvStore, KvStoreOps,
    KvWatchEventType, KvWatchResponse, List, ListOption, Put, PutOperation, PutOption, PutResult,
    Watch, WatchOption,
};
use toy_api_server::store::{error::StoreError, StoreConnection};
use toy_api_server::task::store::{Pending, TaskStore, TaskStoreOps, WatchPending};
use toy_h::HttpClient;
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;
use tracing::{instrument, span, Instrument, Level};

#[derive(Clone, Debug)]
pub struct EtcdStore<T> {
    con: Option<EtcdStoreConnection<T>>,
}

#[derive(Clone, Debug)]
pub struct EtcdStoreConnection<T> {
    client: Client<T>,
}

#[derive(Clone, Debug)]
pub struct EtcdStoreOps<T> {
    _t: PhantomData<T>,
}

impl<T> EtcdStore<T> {
    pub fn new() -> Self {
        EtcdStore { con: None }
    }
}

impl<T> EtcdStore<T>
where
    T: HttpClient,
{
    fn establish_etcd(&mut self, client: T, store_name: &'static str) -> Result<(), StoreError> {
        if self.con.is_some() {
            return Ok(());
        }

        let host = std::env::var("TOY_STORE_ETCD_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("TOY_STORE_ETCD_PORT").unwrap_or_else(|_| "2379".to_string());
        let url = format!("http://{}:{}", host, port);
        tracing::info!("toy {} store=etcd. connecting:{}", store_name, &url);
        match Client::new(client, url) {
            Ok(c) => {
                self.con = Some(EtcdStoreConnection { client: c });
            }
            Err(e) => return Err(StoreError::error(e)),
        };
        Ok(())
    }
}

impl<T> StoreConnection for EtcdStoreConnection<T> where T: HttpClient {}

impl<T> TaskStoreOps<EtcdStoreConnection<T>> for EtcdStoreOps<T> where T: HttpClient + 'static {}
impl<T> KvStoreOps<EtcdStoreConnection<T>> for EtcdStoreOps<T> where T: HttpClient + 'static {}

impl<T> TaskStore<T> for EtcdStore<T>
where
    T: HttpClient + 'static,
{
    type Con = EtcdStoreConnection<T>;
    type Ops = EtcdStoreOps<T>;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        EtcdStoreOps { _t: PhantomData }
    }

    fn establish(&mut self, client: T) -> Result<(), StoreError> {
        self.establish_etcd(client, "task")
    }
}

impl<T> KvStore<T> for EtcdStore<T>
where
    T: HttpClient + 'static,
{
    type Con = EtcdStoreConnection<T>;
    type Ops = EtcdStoreOps<T>;

    fn con(&self) -> Option<Self::Con> {
        self.con.clone()
    }

    fn ops(&self) -> Self::Ops {
        EtcdStoreOps { _t: PhantomData }
    }

    fn establish(&mut self, client: T) -> Result<(), StoreError> {
        self.establish_etcd(client, "common key value")
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> Find for EtcdStoreOps<T>
where
    T: HttpClient,
{
    type Con = EtcdStoreConnection<T>;

    #[instrument(skip(self, con))]
    async fn find<V>(
        &self,
        con: Self::Con,
        key: String,
        opt: FindOption,
    ) -> Result<Option<KvResponse<V>>, StoreError>
    where
        V: DeserializableOwned,
    {
        tracing::debug!("find key:{:?}", key);
        let res = con.client.get(&key).await?.value()?;
        match res {
            Some(v) => {
                let version = v.version();
                let r = toy_pack_json::unpack::<V>(&v.into_data())?;
                Ok(Some(KvResponse::with_version(r, version)))
            }
            None => {
                tracing::debug!("[find] not found. key:{:?}, opt:{:?}", key, opt);
                Ok(Option::<KvResponse<V>>::None)
            }
        }
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> List for EtcdStoreOps<T>
where
    T: HttpClient,
{
    type Con = EtcdStoreConnection<T>;

    #[instrument(skip(self, con))]
    async fn list<V>(
        &self,
        con: Self::Con,
        prefix: String,
        _opt: ListOption,
    ) -> Result<Vec<V>, StoreError>
    where
        V: DeserializableOwned,
    {
        tracing::debug!("list prefix:{:?}", prefix);
        con.client
            .list(&prefix)
            .await?
            .unpack(|x| toy_pack_json::unpack::<V>(&x.into_data()).map_err(|e| e.into()))
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> Put for EtcdStoreOps<T>
where
    T: HttpClient,
{
    type Con = EtcdStoreConnection<T>;

    #[instrument(skip(self, con, v))]
    async fn put<V>(
        &self,
        con: Self::Con,
        key: String,
        v: V,
        opt: PutOption,
    ) -> Result<PutResult, StoreError>
    where
        V: Serializable + Send,
    {
        async fn insert<C>(
            con: &EtcdStoreConnection<C>,
            key: &String,
            s: &String,
            _opt: &PutOption,
        ) -> Result<PutResult, StoreError>
        where
            C: HttpClient,
        {
            let r = con.client.create(key, s).await;
            match r {
                Ok(tx) if tx.is_success() => Ok(PutResult::Create),
                Ok(tx) if !tx.is_success() => Err(StoreError::allready_exists(key)),
                Err(e) => Err(StoreError::error(e)),
                _ => unreachable!(),
            }
        }

        async fn update<C>(
            con: &EtcdStoreConnection<C>,
            key: &String,
            s: &String,
            opt: &PutOption,
        ) -> Result<PutResult, StoreError>
        where
            C: HttpClient,
        {
            let version = if opt.version() != 0 {
                opt.version()
            } else {
                let res = con.client.get(key).await?.value()?;
                match res {
                    Some(v) => v.version(),
                    None => return Err(StoreError::not_found_update_target(key)),
                }
            };

            let upd_res = con.client.update(key, s, version).await?;
            if upd_res.is_success() {
                Ok(PutResult::Update)
            } else {
                Err(StoreError::failed_opration("update", key, ""))
            }
        }

        tracing::debug!("put key:{:?}", key);
        let s = toy_pack_json::pack_to_string(&v)?;

        match opt.operation() {
            PutOperation::Fill => match insert(&con, &key, &s, &opt).await {
                Ok(r) => Ok(r),
                Err(e) => match e {
                    StoreError::AllreadyExists { .. } => update(&con, &key, &s, &opt).await,
                    _ => Err(e),
                },
            },
            PutOperation::CreateOnly => insert(&con, &key, &s, &opt).await,
            PutOperation::UpdatedOnly => update(&con, &key, &s, &opt).await,
        }
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> Delete for EtcdStoreOps<T>
where
    T: HttpClient,
{
    type Con = EtcdStoreConnection<T>;

    #[instrument(skip(self, con))]
    async fn delete(
        &self,
        con: Self::Con,
        key: String,
        _opt: DeleteOption,
    ) -> Result<DeleteResult, StoreError> {
        tracing::debug!("delete key:{:?}", key);
        let single_res = con.client.get(&key).await?.value()?;
        match single_res {
            Some(v) => {
                let rm_res = con.client.remove(&key, v.version()).await?;
                if !rm_res.is_success() {
                    Err(StoreError::failed_opration("delete", &key, ""))
                } else {
                    Ok(DeleteResult::Deleted)
                }
            }
            None => Ok(DeleteResult::NotFound),
        }
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> Watch for EtcdStoreOps<T>
where
    T: HttpClient,
{
    type Con = EtcdStoreConnection<T>;

    #[instrument(skip(self, con))]
    async fn watch<V>(
        &self,
        con: Self::Con,
        prefix: String,
        _opt: WatchOption,
    ) -> Result<BoxStream<Result<KvWatchResponse<V>, StoreError>>, StoreError>
    where
        V: DeserializableOwned,
    {
        let stream = con.client.watch(prefix).await?;
        Ok(stream
            .map(|x: Result<WatchResponse, StoreError>| match x {
                Ok(res) => {
                    let values = res.unpack(|x, e| {
                        let version = x.version();
                        let value = toy_pack_json::unpack::<V>(&x.into_data()).ok()?;
                        let evt = match e {
                            EventType::PUT => KvWatchEventType::PUT,
                            EventType::DELETE => KvWatchEventType::DELETE,
                        };
                        Some(Ok((KvResponse::with_version(value, version), evt)))
                    })?;
                    Ok(KvWatchResponse::new(values.into_iter()))
                }
                Err(e) => Err(e),
            })
            .boxed())
    }
}

#[toy_api_server::async_trait::async_trait]
impl<T> Pending for EtcdStoreOps<T>
where
    T: HttpClient,
{
    type Con = EtcdStoreConnection<T>;

    #[instrument(skip(self, con))]
    async fn pending(&self, con: Self::Con, key: String, v: PendingTask) -> Result<(), StoreError> {
        let s = toy_pack_json::pack_to_string(&v)?;
        let create_res = con.client.create(&key, &s).await?;
        if create_res.is_success() {
            Ok(())
        } else {
            Err(StoreError::allready_exists(key))
        }
    }
}

impl<T> WatchPending for EtcdStoreOps<T>
where
    T: HttpClient + 'static,
{
    type Con = EtcdStoreConnection<T>;
    type Stream = impl toy_h::Stream<Item = Result<Vec<PendingTask>, StoreError>> + Send + 'static;
    type T = impl Future<Output = Result<Self::Stream, StoreError>> + Send + 'static;

    fn watch_pending(&self, con: Self::Con, prefix: String) -> Self::T {
        let span = span!(Level::DEBUG, "watch", prefix = ?prefix);
        let prefix = prefix.clone();
        async move {
            let stream = con.client.watch(prefix).await?;
            Ok(stream.map(|x: Result<WatchResponse, StoreError>| match x {
                Ok(res) => res.unpack(|x, e| match e {
                    EventType::PUT => Some(
                        toy_pack_json::unpack::<PendingTask>(&x.into_data()).map_err(|e| e.into()),
                    ),
                    _ => None,
                }),
                Err(e) => Err(e),
            }))
        }
        .instrument(span)
    }
}

use crate::common;
use crate::store::kv::{
    KvStore, KvWatchEventType, List, ListOption, Put, PutOption, PutResult, Watch, WatchOption,
};
use crate::ApiError;
use futures_util::{stream, TryStreamExt};
use std::net::SocketAddr;
use toy_api::common::{Format, PostOption};
use toy_api::supervisors::{Supervisor, SupervisorName};
use toy_api::task::PendingTask;
use toy_h::HttpClient;

pub async fn dispatch_task<T, Store>(store: Store, client: T) -> Result<(), ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    match store
        .ops()
        .watch::<PendingTask>(
            store.con().unwrap(),
            common::constants::PENDINGS_KEY_PREFIX.to_string(),
            WatchOption::new(),
        )
        .await
    {
        Ok(st) => {
            let _ = st
                .map_err(|e| ApiError::error(e))
                .try_for_each(|r| async {
                    stream::iter(r.into_values().into_iter().map(Ok))
                        .try_for_each(|v| async {
                            let version = v.version();
                            match v.event() {
                                KvWatchEventType::DELETE => Ok(()),
                                KvWatchEventType::PUT => {
                                    let task = v.into_value();
                                    let sv = candidate(&store).await?;
                                    let task = allocate(&store, sv.name(), task, version).await?;
                                    request(&client, task, sv.addr()).await
                                }
                            }
                        })
                        .await
                })
                .await;
            Ok(())
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

async fn candidate<Store, T>(store: &Store) -> Result<Supervisor, ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    match store
        .ops()
        .list::<Supervisor>(
            store.con().unwrap(),
            common::constants::SUPERVISORS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(vec) => {
            let s = vec.into_iter().filter(|x| x.is_alive()).nth(0);
            if let Some(s) = s {
                Ok(s)
            } else {
                Err(ApiError::error("supervisor not found."))
            }
        }
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

async fn allocate<Store, T>(
    store: &Store,
    name: &SupervisorName,
    v: PendingTask,
    version: u64,
) -> Result<PendingTask, ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    let allocated = v.allocate(name, chrono::Utc::now());
    match store
        .ops()
        .put(
            store.con().unwrap(),
            common::constants::pending_key(allocated.task_id()),
            allocated.clone(),
            PutOption::new().with_update_only().with_version(version),
        )
        .await
    {
        Ok(PutResult::Update) => Ok(allocated),
        Ok(_) => unreachable!(),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

async fn request<T>(client: &T, v: PendingTask, addr: SocketAddr) -> Result<(), ApiError>
where
    T: HttpClient,
{
    toy_api_http_common::request::post(
        client,
        None,
        &format!("http://{}:{}", addr.ip(), addr.port()),
        "tasks",
        &v,
        PostOption::new().with_format(Format::MessagePack),
    )
    .await
    .map_err(|e| e.into())
}

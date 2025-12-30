use crate::common;
use crate::store::kv::{KvStore, List, ListOption, Put, PutOption, PutResult};
use crate::ApiError;
use toy_api::actors::Actor;
use toy_api::common::{Format, PostOption};
use toy_api::task::{AllocateResponse, PendingTask};
use toy_h::HttpClient;

pub async fn dispatch_task<T, Store>(
    store: Store,
    client: T,
    interval_mills: u64,
) -> Result<(), ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    loop {
        tracing::debug!("check pending task");

        toy_rt::sleep(interval_mills).await;
        match store
            .ops()
            .list::<PendingTask>(
                store.con().unwrap(),
                common::constants::PENDINGS_KEY_PREFIX.to_string(),
                ListOption::new(),
            )
            .await
        {
            Ok(vec) => {
                let filterd = vec
                    .into_iter()
                    .filter(|x| x.value().is_dispatchable())
                    .collect::<Vec<_>>();
                if filterd.len() > 0 {
                    tracing::info!("found pending task, {}", filterd.len());
                    for task in filterd {
                        let version = task.version();
                        match execute(&store, &client, task.into_value(), version).await {
                            Ok(r) => {
                                if r.is_ok() {
                                    tracing::info!("requested {}", r.task_id());
                                } else {
                                    tracing::info!("no result {}", r.task_id());
                                }
                            }
                            Err(e) => {
                                tracing::error!("request failed cause {:?}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
    }
}

async fn execute<T, Store>(
    store: &Store,
    client: &T,
    task: PendingTask,
    version: u64,
) -> Result<AllocateResponse, ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    let sv = candidate(store).await?;
    if sv.is_none() {
        return Ok(AllocateResponse::none(task.task_id()));
    }

    let sv = sv.unwrap();
    let (task, new_version) =
        put_pending_task(store, task.allocate(sv.name(), chrono::Utc::now()), version).await?;

    tracing::info!("request task to {}:{}", sv.name(), sv.addr().port());

    match toy_api_http_common::request::post(
        client,
        None,
        &format!("https://{}:{}", sv.name(), sv.addr().port()),
        "tasks",
        &task,
        PostOption::new().with_format(Format::MessagePack),
    )
    .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            let _ = put_pending_task(
                store,
                task.allocate_failed(sv.name(), chrono::Utc::now()),
                new_version,
            )
            .await?;
            Err(e.into())
        }
    }
}

async fn candidate<Store, T>(store: &Store) -> Result<Option<Actor>, ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    match store
        .ops()
        .list::<Actor>(
            store.con().unwrap(),
            common::constants::ACTORS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(vec) => {
            let s = vec.into_iter().filter(|x| x.value().is_alive()).nth(0);
            if let Some(s) = s {
                Ok(Some(s.into_value()))
            } else {
                tracing::info!("candidate actor not found.");
                Ok(None)
            }
        }
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

async fn put_pending_task<Store, T>(
    store: &Store,
    v: PendingTask,
    version: u64,
) -> Result<(PendingTask, u64), ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    match store
        .ops()
        .put(
            store.con().unwrap(),
            common::constants::pending_key(v.task_id()),
            v.clone(),
            PutOption::new().with_update_only().with_version(version),
        )
        .await
    {
        Ok(PutResult::Update(new_version)) => Ok((v, new_version)),
        Ok(_) => unreachable!(),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

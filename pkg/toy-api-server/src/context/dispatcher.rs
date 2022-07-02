use crate::common;
use crate::store::kv::{KvStore, List, ListOption, Put, PutOption, PutResult};
use crate::ApiError;
use toy_api::common::{Format, PostOption};
use toy_api::supervisors::{Supervisor, SupervisorName};
use toy_api::task::{AllocateResponse, PendingTask};
use toy_h::HttpClient;

pub async fn dispatch_task<T, Store>(store: Store, client: T) -> Result<(), ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    loop {
        tracing::debug!("check pending task");
        // 3sec
        toy_rt::sleep(3000).await;
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
                    tracing::debug!("found pending task, {}", filterd.len());
                    for task in filterd {
                        let version = task.version();
                        match execute(&store, &client, task.into_value(), version).await {
                            Ok(r) => {
                                tracing::debug!("execute task {}", r.task_id());
                            }
                            Err(e) => {
                                tracing::error!("{:?}", e);
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
    let task = allocate(store, sv.name(), task, version).await?;

    toy_api_http_common::request::post(
        client,
        None,
        &format!("http://{}:{}", sv.addr().ip(), sv.addr().port()),
        "tasks",
        &task,
        PostOption::new().with_format(Format::MessagePack),
    )
    .await
    .map_err(|e| e.into())
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
            let s = vec.into_iter().filter(|x| x.value().is_alive()).nth(0);
            if let Some(s) = s {
                Ok(s.into_value())
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

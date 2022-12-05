use crate::common::constants;
use crate::context::Context;
use crate::store::kv::{KvStore, Put, PutOption, PutResult, Update, UpdateResult};
use crate::store::task_event::{
    CreateOption, FindOption, ListOption, TaskEventStore, TaskEventStoreOps,
};
use crate::ApiError;
use chrono::{Duration, Utc};
use toy_api::common::{self as api_common, CommonPostResponse, ListOptionLike};
use toy_api::graph::Graph;
use toy_api::selection::selector::Selector;
use toy_api::task::{FinishResponse, PendingResult, PendingTask, TaskEvent, TaskListOption};
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;
use toy_api_http_common::{codec, reply};
use toy_core::task::TaskId;
use toy_h::HttpClient;

pub async fn post<T>(
    ctx: Context,
    opt: api_common::PostOption,
    request: Bytes,
    store: &impl KvStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let format = opt.format();
    let v = codec::decode::<_, Graph>(request, format)?;

    /*
     validate...
    */

    let id = TaskId::new();
    let pending = PendingTask::new(id, v);
    let key = constants::pending_key(id);
    match store
        .ops()
        .put(
            store.con().unwrap(),
            key,
            pending,
            PutOption::new().with_create_only(),
        )
        .await
    {
        Ok(PutResult::Create) => Ok(reply::into_response(
            &(PendingResult::from_id(id)),
            format,
            None,
        )),
        Ok(PutResult::Update(_)) => unreachable!(),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

pub async fn finish<T>(
    ctx: Context,
    store: &impl KvStore<T>,
    key: String,
    api_opt: api_common::PostOption,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let id = TaskId::parse_str(&key);
    if id.is_err() {
        return Err(ApiError::task_id_invalid_format(key));
    }
    let id = id.unwrap();

    let format = api_opt.format();
    let key = constants::generate_key(constants::PENDINGS_KEY_PREFIX, key);
    let now = Utc::now();
    let f = |v: PendingTask| Some(v.finished(now));
    match store.ops().update(store.con().unwrap(), key, f).await {
        Ok(UpdateResult::Update(_)) => {
            Ok(reply::into_response(&FinishResponse::ok(id), format, None))
        }
        Ok(UpdateResult::NotFound) => Ok(reply::into_response(
            &FinishResponse::not_found(id),
            format,
            None,
        )),
        Ok(UpdateResult::None) => unreachable!(),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

pub async fn list<T>(
    ctx: Context,
    opt: TaskListOption,
    log_store: &impl TaskEventStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let dt = opt.timestamp().unwrap_or(Utc::now() - Duration::days(1));
    let store_opt = ListOption::new()
        .with_field_selection(Selector::default().greater_than("timestamp", dt.to_rfc3339()));

    let format = opt.common().format();
    match log_store
        .ops()
        .list(log_store.con().unwrap(), store_opt)
        .await
    {
        Ok(v) => Ok(reply::into_response(&v, format, None)),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

pub async fn post_task_event<T>(
    ctx: Context,
    opt: api_common::PostOption,
    request: Bytes,
    event_store: &impl TaskEventStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let format = opt.format();
    let v = codec::decode::<_, Vec<TaskEvent>>(request, format)?;

    match event_store
        .ops()
        .create(event_store.con().unwrap(), v, CreateOption::new())
        .await
    {
        Ok(()) => {
            let r = CommonPostResponse::with_code(StatusCode::CREATED.as_u16());
            let r = reply::into_response(&r, format, opt.indent());
            Ok((StatusCode::CREATED, r))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

pub async fn find_task_event<T>(
    ctx: Context,
    key: String,
    opt: toy_api::common::FindOption,
    event_store: &impl TaskEventStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let id = match TaskId::parse_str(&key) {
        Ok(id) => id,
        Err(_) => return Err(ApiError::task_id_invalid_format(key.to_string())),
    };

    match event_store
        .ops()
        .find(event_store.con().unwrap(), id, FindOption::new())
        .await
    {
        Ok(Some(v)) => Ok(reply::into_response(&v, opt.format(), opt.indent())),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

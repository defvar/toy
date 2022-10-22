use crate::context::Context;
use crate::task::store::{List, ListOption, Pending, TaskLogStore, TaskStore};
use crate::{common, ApiError};
use chrono::{Duration, Utc};
use toy_api::common::{ListOptionLike, PostOption};
use toy_api::graph::Graph;
use toy_api::selection::field::Selection;
use toy_api::task::{LogOption, PendingResult, PendingTask, TaskListOption};
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;
use toy_api_http_common::{codec, reply};
use toy_core::task::TaskId;
use toy_h::HttpClient;

pub async fn post<T>(
    ctx: Context,
    opt: PostOption,
    request: Bytes,
    store: &impl TaskStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let format = opt.format();
    let v = codec::decode::<_, Graph>(request, format)?;

    /*
     validate...
    */

    let id = TaskId::new();
    let pending = PendingTask::new(id, v);
    let key = common::constants::pending_key(id);
    match store
        .ops()
        .pending(store.con().unwrap(), key, pending)
        .await
    {
        Ok(()) => Ok(reply::into_response(
            &(PendingResult::from_id(id)),
            format,
            None,
        )),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

pub async fn list<T>(
    ctx: Context,
    opt: TaskListOption,
    log_store: &impl TaskLogStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let dt = opt.timestamp().unwrap_or(Utc::now() - Duration::days(1));
    let store_opt = ListOption::new()
        .with_field_selection(Selection::default().greater_than("timestamp", dt.to_rfc3339()));

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

pub async fn log<T>(
    ctx: Context,
    key: String,
    opt: LogOption,
    log_store: &impl TaskLogStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    use crate::task::store::{FindLog, FindOption};
    let id = match TaskId::parse_str(&key) {
        Ok(id) => id,
        Err(_) => return Err(ApiError::task_id_invalid_format(key.to_string())),
    };

    let format = opt.format();
    match log_store
        .ops()
        .find(log_store.con().unwrap(), id, FindOption::new())
        .await
    {
        Ok(Some(v)) => Ok(reply::into_response(&v, format, None)),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

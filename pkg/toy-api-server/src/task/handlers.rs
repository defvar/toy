use crate::context::Context;
use crate::task::store::{List, ListOption, Pending, TaskLogStore, TaskStore};
use crate::{common, ApiError};
use chrono::{Duration, Utc};
use std::convert::Infallible;
use toy_api::common::ListOptionLike;
use toy_api::graph::Graph;
use toy_api::selection::field::Selection;
use toy_api::task::{LogOption, PendingResult, PendingTask, PostOption, TaskListOption};
use toy_api_http_common::{codec, reply};
use toy_core::task::TaskId;
use toy_h::HttpClient;
use warp::http::StatusCode;
use warp::hyper::body::Bytes;
use warp::reply::Reply;

pub async fn post<T>(
    ctx: Context,
    opt: Option<PostOption>,
    request: Bytes,
    store: impl TaskStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let format = opt.map(|x| x.format()).unwrap_or(None);
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
        Ok(()) => {
            Ok(reply::into_response(&(PendingResult::from_id(id)), format, None).into_response())
        }
        Err(e) => Err(ApiError::store_operation_failed(e).into_rejection()),
    }
}

pub async fn tasks<T>(
    ctx: Context,
    opt: Option<TaskListOption>,
    log_store: impl TaskLogStore<T>,
) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let dt = match opt {
        Some(ref o) => o
            .timestamp()
            .map(|x| x.clone())
            .unwrap_or(Utc::now() - Duration::days(1)),
        None => Utc::now() - Duration::days(1),
    };

    let store_opt = ListOption::new()
        .with_field_selection(Selection::default().greater_than("timestamp", dt.to_rfc3339()));

    let format = opt.map(|x| x.common().format()).unwrap_or(None);
    match log_store
        .ops()
        .list(log_store.con().unwrap(), store_opt)
        .await
    {
        Ok(v) => Ok(reply::into_response(&v, format, None)),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn log<T>(
    key: String,
    ctx: Context,
    opt: Option<LogOption>,
    log_store: impl TaskLogStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    use crate::task::store::{FindLog, FindOption};
    let id = match TaskId::parse_str(&key) {
        Ok(id) => id,
        Err(_) => return Err(ApiError::task_id_invalid_format(key.to_string()).into_rejection()),
    };

    let format = opt.map(|x| x.format()).unwrap_or(None);
    match log_store
        .ops()
        .find(log_store.con().unwrap(), id, FindOption::new())
        .await
    {
        Ok(Some(v)) => Ok(reply::into_response(&v, format, None)),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

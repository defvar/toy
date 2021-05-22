use crate::context::Context;
use crate::store::kv::{Find, FindOption, Put, PutOption, PutResult};
use crate::task::store::{List, ListOption, Pending, TaskLogStore, TaskStore, WatchPending};
use crate::{common, ApiError};
use futures_util::StreamExt;
use std::convert::Infallible;
use toy_api::graph::Graph;
use toy_api::task::{
    AllocateOption, AllocateRequest, AllocateResponse, ListOption as ApiListOption, LogOption,
    PendingEntity, PendingResult, PendingStatus, PendingsEntity, PostOption, WatchOption,
};
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
    let v = common::body::decode::<_, Graph>(request, format)?;

    /*
     validate...
    */

    let id = TaskId::new();
    let pending = PendingEntity::new(id, v);
    let key = common::constants::pending_key(id);
    match store
        .ops()
        .pending(store.con().unwrap(), key, pending)
        .await
    {
        Ok(()) => {
            Ok(common::reply::into_response(&(PendingResult::from_id(id)), format).into_response())
        }
        Err(e) => Err(ApiError::store_operation_failed(e).into_rejection()),
    }
}

pub async fn watch<T>(
    ctx: Context,
    opt: Option<WatchOption>,
    store: impl TaskStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let format = opt.map(|x| x.format()).unwrap_or(None);

    match store
        .ops()
        .watch_pending(
            store.con().unwrap(),
            common::constants::PENDINGS_KEY_PREFIX.to_string(),
        )
        .await
    {
        Ok(stream) => {
            let stream = stream.map(move |x| match x {
                Ok(v) => {
                    let pendings = PendingsEntity::new(v);
                    common::reply::encode(&pendings, format)
                }
                Err(_) => Err(ApiError::error("failed get stream data")),
            });
            Ok(common::reply::into_response_stream(stream, format).into_response())
        }
        Err(e) => Err(ApiError::store_operation_failed(e).into_rejection()),
    }
}

pub async fn allocate<T>(
    key: String,
    ctx: Context,
    opt: Option<AllocateOption>,
    request: Bytes,
    store: impl TaskStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let id = match TaskId::parse_str(&key) {
        Ok(id) => id,
        Err(_) => return Err(ApiError::task_id_invalid_format(key.to_string()).into_rejection()),
    };

    let format = opt.map(|x| x.format()).unwrap_or(None);
    let request = common::body::decode::<_, AllocateRequest>(request, format)?;

    // get with version....
    match store
        .ops()
        .find::<PendingEntity>(
            store.con().unwrap(),
            common::constants::pending_key(id),
            FindOption::new(),
        )
        .await
    {
        Ok(Some(v)) => {
            match v.status() {
                PendingStatus::Allocated => {
                    return Ok(common::reply::into_response(
                        &AllocateResponse::not_found(id),
                        format,
                    ))
                }
                _ => (),
            }
            let allocated = v.allocate(request.name(), chrono::Utc::now().to_rfc3339());
            match store
                .ops()
                .put(
                    store.con().unwrap(),
                    common::constants::pending_key(id),
                    allocated,
                    PutOption::new().with_update_only(), //set version....
                )
                .await
            {
                Ok(PutResult::Update) => Ok(common::reply::into_response(
                    &AllocateResponse::ok(id),
                    format,
                )),
                Ok(_) => unreachable!(),
                Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
            }
        }
        Ok(None) => Ok(common::reply::into_response(
            &AllocateResponse::not_found(id),
            format,
        )),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn tasks<T>(
    ctx: Context,
    opt: Option<ApiListOption>,
    log_store: impl TaskLogStore<T>,
) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let format = opt.map(|x| x.format()).unwrap_or(None);
    match log_store
        .ops()
        .list(log_store.con().unwrap(), ListOption::new())
        .await
    {
        Ok(v) => Ok(common::reply::into_response(&v, format)),
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
        Ok(Some(v)) => Ok(common::reply::into_response(&v, format)),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

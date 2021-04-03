use crate::common;
use crate::store::kv::{Find, FindOption, Put, PutOption, PutResult};
use crate::task::store::{List, ListOption, Pending, TaskLogStore, TaskStore, WatchPending};
use futures_util::StreamExt;
use std::convert::Infallible;
use toy_api::graph::GraphEntity;
use toy_api::task::{
    AllocateOption, AllocateRequest, AllocateResponse, PendingEntity, PendingResult, PendingStatus,
    PendingsEntity,
};
use toy_core::task::TaskId;
use toy_h::HttpClient;
use toy_pack_json::EncodeError;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn post<T>(
    v: GraphEntity,
    store: impl TaskStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
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
        Ok(()) => Ok(common::reply::json(&(PendingResult::from_id(id))).into_response()),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn watch<T>(store: impl TaskStore<T>) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    match store
        .ops()
        .watch_pending(
            store.con().unwrap(),
            common::constants::PENDINGS_KEY_PREFIX.to_string(),
        )
        .await
    {
        Ok(stream) => {
            let stream = stream.map(|x| match x {
                Ok(v) => {
                    let pendings = PendingsEntity::new(v);
                    toy_pack_json::pack_to_string(&pendings)
                }
                Err(_) => Err(EncodeError::error("failed get stream data")),
            });
            Ok(common::reply::json_stream(stream).into_response())
        }
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn allocate<T>(
    key: String,
    opt: Option<AllocateOption>,
    request: AllocateRequest,
    store: impl TaskStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let id = match TaskId::parse_str(&key) {
        Ok(id) => id,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    let format = opt.map(|x| x.format()).unwrap_or(None);

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
            let a = v.allocate(request.name(), chrono::Utc::now().to_rfc3339());
            match store
                .ops()
                .put(
                    store.con().unwrap(),
                    common::constants::pending_key(id),
                    a,
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

pub async fn tasks<T>(log_store: impl TaskLogStore<T>) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    match log_store
        .ops()
        .list(log_store.con().unwrap(), ListOption::new())
        .await
    {
        Ok(v) => Ok(common::reply::json(&v).into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn log<T>(
    key: String,
    log_store: impl TaskLogStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    use crate::task::store::{FindLog, FindOption};
    let id = match TaskId::parse_str(key) {
        Ok(id) => id,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    match log_store
        .ops()
        .find(log_store.con().unwrap(), id, FindOption::new())
        .await
    {
        Ok(Some(v)) => Ok(common::reply::json(&v).into_response()),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

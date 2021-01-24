use crate::common;
use crate::common::models::GraphEntity;
use crate::task::models::{PendingEntity, PendingResult, RunningTasksEntity};
use crate::task::store::{
    Find, FindOption, List, ListOption, Pending, TaskLogStore, TaskStore, WatchPending,
};
use futures_util::StreamExt;
use std::convert::Infallible;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::core::oneshot;
use toy::core::task::TaskId;
use toy::supervisor::{Request, TaskResponse};
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

    let pending = PendingEntity::new(v);
    let id = TaskId::new();
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
                Ok(v) => toy_pack_json::pack_to_string(&v),
                Err(_) => Err(EncodeError::error("failed get stream data")),
            });
            Ok(common::reply::json_stream(stream).into_response())
        }
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn running_tasks(
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, Infallible> {
    let (tx_, rx_) = oneshot::channel::<Vec<TaskResponse>, ServiceError>();
    let _ = tx.send_ok(Request::Tasks(tx_)).await;
    if let Some(Ok(r)) = rx_.recv().await {
        let vec = RunningTasksEntity::from(r);
        Ok(common::reply::json(&vec).into_response())
    } else {
        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
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

// fn test_stream() -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
//     let mut counter: u64 = 0;
//     let event_stream = tokio::time::interval(Duration::from_secs(1)).map(move |_| {
//         counter += 1;
//         sse_counter(counter)
//     });
//     event_stream
// }
//
// fn sse_counter(counter: u64) -> Result<impl ServerSentEvent, Infallible> {
//     Ok(warp::sse::data(counter))
// }

use crate::common;
use crate::task::models::RunningTasksEntity;
use crate::task::store::{Find, FindOption, List, ListOption, TaskLogStore};
use std::convert::Infallible;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::core::oneshot;
use toy::core::task::TaskId;
use toy::supervisor::{Request, TaskResponse};
use warp::http::StatusCode;
use warp::reply::Reply;

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

pub async fn tasks(log_store: impl TaskLogStore) -> Result<impl warp::Reply, Infallible> {
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

pub async fn log(
    key: String,
    log_store: impl TaskLogStore,
) -> Result<impl warp::Reply, warp::Rejection> {
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

use crate::common;
use crate::store::{Find, FindOption, StoreConnection, StoreOpsFactory};
use crate::task::models::{ListTaskEntity, RunTaskEntity};
use std::convert::Infallible;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;
use toy_core::mpsc::Outgoing;
use toy_core::oneshot;
use toy_core::prelude::Value;
use toy_core::supervisor::{Request, RunTaskResponse, TaskResponse};
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list(mut tx: Outgoing<Request, ServiceError>) -> Result<impl warp::Reply, Infallible> {
    let (tx_, rx_) = oneshot::channel::<Vec<TaskResponse>, ServiceError>();
    let _ = tx.send_ok(Request::Tasks(tx_)).await;
    if let Some(Ok(r)) = rx_.recv().await {
        let vec = ListTaskEntity::from(r);
        Ok(common::reply::json(&vec).into_response())
    } else {
        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

pub async fn run<C>(
    key: String,
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = common::constants::graph_key(key);
    match ops.find(con, key, FindOption::new()).await {
        Ok(Some(v)) => {
            let v = match toy_pack_json::unpack::<Value>(&v) {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("error:{:?}", e);
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                }
            };
            let graph = match Graph::from(v) {
                Ok(g) => g,
                Err(e) => {
                    tracing::error!("error:{:?}", e);
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                }
            };
            let (tx_, rx_) = oneshot::channel::<RunTaskResponse, ServiceError>();
            let _ = tx.send_ok(Request::RunTask(graph, tx_)).await;
            if let Some(Ok(r)) = rx_.recv().await {
                Ok(common::reply::json(&(RunTaskEntity::from(r))).into_response())
            } else {
                Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

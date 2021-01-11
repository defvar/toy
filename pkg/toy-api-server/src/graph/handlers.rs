use super::store::{
    DeleteOption, DeleteResult, FindOption, GraphStoreOps, ListOption, PutOption, PutResult,
};
use crate::common;
use crate::graph::models::{GraphEntity, RunTaskEntity};
use crate::store::StoreConnection;
use std::convert::Infallible;
use toy::core::error::ServiceError;
use toy::core::graph::Graph;
use toy::core::mpsc::Outgoing;
use toy::core::oneshot;
use toy::supervisor::{Request, RunTaskResponse};
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<C>(
    con: C,
    ops: impl GraphStoreOps<C>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    //let ops = ops.create().unwrap();
    match ops
        .list(
            con,
            common::constants::GRAPHS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => Ok(common::reply::json(&v).into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn find<C>(
    key: String,
    con: C,
    ops: impl GraphStoreOps<C>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let key = common::constants::graph_key(key);
    match ops.find(con, key, FindOption::new()).await {
        Ok(v) => match v {
            Some(v) => Ok(common::reply::json(&v).into_response()),
            None => Ok(StatusCode::NOT_FOUND.into_response()),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn put<C>(
    key: String,
    v: GraphEntity,
    con: C,
    ops: impl GraphStoreOps<C>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let key = common::constants::graph_key(key);
    //
    // validation...?
    //
    match ops.put(con, key, v, PutOption::new()).await {
        Ok(r) => match r {
            PutResult::Create => Ok(StatusCode::CREATED),
            PutResult::Update => Ok(StatusCode::OK),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete<C>(
    key: String,
    con: C,
    ops: impl GraphStoreOps<C>,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let key = common::constants::graph_key(key);
    match ops.delete(con, key, DeleteOption::new()).await {
        Ok(r) => match r {
            DeleteResult::Deleted => Ok(StatusCode::NO_CONTENT),
            DeleteResult::NotFound => Ok(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn run<C>(
    key: String,
    con: C,
    ops: impl GraphStoreOps<C>,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let key = common::constants::graph_key(key);
    match ops.find(con, key, FindOption::new()).await {
        Ok(Some(v)) => {
            let v = match toy::core::data::pack(&v) {
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

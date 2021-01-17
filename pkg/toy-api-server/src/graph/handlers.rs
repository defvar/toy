use super::store::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult,
};
use crate::common;
use crate::graph::models::{GraphEntity, RunTaskEntity};
use crate::graph::store::GraphStore;
use std::convert::Infallible;
use toy::core::error::ServiceError;
use toy::core::graph::Graph;
use toy::core::mpsc::Outgoing;
use toy::core::oneshot;
use toy::supervisor::{Request, RunTaskResponse};
use toy_h::HttpClient;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<T>(store: impl GraphStore<T>) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    match store
        .ops()
        .list(
            store.con().unwrap(),
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

pub async fn find<T>(
    key: String,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
    match store
        .ops()
        .find(store.con().unwrap(), key, FindOption::new())
        .await
    {
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

pub async fn put<T>(
    key: String,
    v: GraphEntity,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
    //
    // validation...?
    //
    match store
        .ops()
        .put(store.con().unwrap(), key, v, PutOption::new())
        .await
    {
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

pub async fn delete<T>(
    key: String,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
    match store
        .ops()
        .delete(store.con().unwrap(), key, DeleteOption::new())
        .await
    {
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

pub async fn run<T>(
    key: String,
    store: impl GraphStore<T>,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
    match store
        .ops()
        .find(store.con().unwrap(), key, FindOption::new())
        .await
    {
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

use crate::common;
use crate::store::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult, StoreConnection, StoreOpsFactory,
};
use std::convert::Infallible;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::Value;
use toy_core::supervisor::Request;
use warp::http::StatusCode;
use warp::reply::Reply;

static GRAPHS_KEY_PREFIX: &'static str = "/graphs";

fn full_key(part: String) -> String {
    format!("{}/{}", GRAPHS_KEY_PREFIX, part)
}

pub async fn list<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    match ops
        .list(con, GRAPHS_KEY_PREFIX.to_string(), ListOption::new())
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
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = full_key(key);
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

pub async fn exec<C>(
    key: String,
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = full_key(key);
    match ops.find(con, key, FindOption::new()).await {
        Ok(Some(v)) => {
            let graph = match Graph::from(v) {
                Ok(g) => g,
                Err(e) => {
                    tracing::error!("error:{:?}", e);
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            tx.send_ok(Request::Task(graph))
                .await
                .map(|_| StatusCode::OK)
                .or_else(|_| Ok(StatusCode::INTERNAL_SERVER_ERROR))
        }
        Ok(None) => Ok(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn put<C>(
    key: String,
    v: Value,
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = full_key(key);
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
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = full_key(key);
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

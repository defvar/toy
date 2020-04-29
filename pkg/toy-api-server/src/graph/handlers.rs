use crate::common;
use crate::persist::error::PersistError;
use crate::persist::GraphRegistry;
use log;
use std::convert::Infallible;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::Value;
use toy_core::supervisor::Request;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list(g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    match g.list().await {
        Ok(graphs) => {
            let graphs: Vec<Value> = graphs.into_iter().map(|x| x.config()).collect();
            Ok(common::reply::yaml(&graphs).into_response())
        }
        Err(e) => {
            log::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn find(name: String, g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    match g.find(&name).await {
        Ok(Some(graph)) => Ok(common::reply::yaml(&graph.config()).into_response()),
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            log::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn exec(
    name: String,
    g: GraphRegistry,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, Infallible> {
    match g.find(&name).await {
        Ok(Some(graph)) => tx
            .send_ok(Request::Task(graph))
            .await
            .map(|_| StatusCode::OK)
            .or_else(|_| Ok(StatusCode::INTERNAL_SERVER_ERROR)),
        Ok(None) => Ok(StatusCode::NOT_FOUND),
        Err(e) => {
            log::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn put(v: Value, g: GraphRegistry) -> Result<impl warp::Reply, warp::Rejection> {
    match Graph::from(v) {
        Ok(graph) => match g.put(graph).await {
            Ok(()) => Ok(StatusCode::OK),
            Err(e) => Err(warp::reject::custom(e)),
        },
        Err(e) => Err(warp::reject::custom(PersistError::from(e))),
    }
}

pub async fn delete(name: String, g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    match g.remove(&name).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Ok(StatusCode::NOT_FOUND),
        Err(e) => {
            log::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

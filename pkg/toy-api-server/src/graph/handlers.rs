use crate::common;
use crate::graph::GraphRegistry;
use std::convert::Infallible;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::Value;
use toy_core::supervisor::Request;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list(g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    let graphs = g.lock().await;
    let graphs: Vec<Value> = graphs.clone().into_iter().map(|x| x.config()).collect();

    Ok(common::reply::yaml(&graphs))
}

pub async fn find(name: String, g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    let graphs = g.lock().await;
    let graph: Option<Value> = graphs
        .clone()
        .into_iter()
        .find(|x| x.name() == name)
        .map(|x| x.config());

    match graph {
        Some(ref g) => Ok(common::reply::yaml(&g).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

pub async fn exec(
    name: String,
    g: GraphRegistry,
    mut tx: Outgoing<Request, ServiceError>,
) -> Result<impl warp::Reply, Infallible> {
    let graphs = g.lock().await;
    let graph: Option<Graph> = graphs.clone().into_iter().find(|x| x.name() == name);

    match graph {
        Some(g) => tx
            .send_ok(Request::Task(g))
            .await
            .map(|_| StatusCode::OK)
            .or_else(|_| Ok(StatusCode::INTERNAL_SERVER_ERROR)),
        None => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(name: String, g: GraphRegistry) -> Result<impl warp::Reply, Infallible> {
    let mut graphs = g.lock().await;

    let len = graphs.len();
    graphs.retain(|g| g.name() != name);

    let deleted = graphs.len() != len;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

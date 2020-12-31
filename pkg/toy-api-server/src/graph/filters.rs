use crate::common::body;
use crate::graph::handlers;
use crate::graph::models::GraphEntity;
use crate::store::{StoreConnection, StoreOpsFactory};
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::supervisor::Request;
use warp::Filter;

pub fn graphs<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
    _tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    graphs_list(con.clone(), ops.clone())
        .or(graphs_find(con.clone(), ops.clone()))
        .or(graphs_put(con.clone(), ops.clone()))
        .or(graphs_delete(con.clone(), ops.clone()))
}

pub fn graphs_list<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs")
        .and(warp::get())
        .and(with_ops(store, ops))
        .and_then(|(a, b)| handlers::list(a, b))
}

pub fn graphs_find<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::get())
        .and(with_ops(store, ops))
        .and_then(|a, (b, c)| handlers::find(a, b, c))
}

pub fn graphs_put<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::put())
        .and(body::json::<GraphEntity>())
        .and(with_ops(store, ops))
        .and_then(|a, b, (c, d)| handlers::put(a, b, c, d))
}

pub fn graphs_delete<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::delete())
        .and(with_ops(store, ops))
        .and_then(|a, (b, c)| handlers::delete(a, b, c))
}

fn with_ops<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = ((C, impl StoreOpsFactory<C> + Clone),), Error = std::convert::Infallible>
       + Clone
where
    C: StoreConnection,
{
    warp::any().map(move || (con.clone(), ops.clone()))
}

use super::store::GraphStoreOps;
use crate::common::body;
use crate::graph::handlers;
use crate::graph::models::GraphEntity;
use crate::store::StoreConnection;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::supervisor::Request;
use warp::Filter;

/// warp filter for graphs api.
pub fn graphs<C>(
    con: C,
    ops: impl GraphStoreOps<C>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    graphs_list(con.clone(), ops.clone())
        .or(graphs_find(con.clone(), ops.clone()))
        .or(graphs_put(con.clone(), ops.clone()))
        .or(graphs_delete(con.clone(), ops.clone()))
        .or(graphs_run(con.clone(), ops.clone(), tx))
}

pub fn graphs_list<C>(
    store: C,
    ops: impl GraphStoreOps<C>,
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
    ops: impl GraphStoreOps<C>,
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
    ops: impl GraphStoreOps<C>,
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
    ops: impl GraphStoreOps<C>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::delete())
        .and(with_ops(store, ops))
        .and_then(|a, (b, c)| handlers::delete(a, b, c))
}

pub fn graphs_run<C>(
    store: C,
    ops: impl GraphStoreOps<C>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String / "run")
        .and(warp::post())
        .and(with_ops_and_tx(store, ops, tx))
        .and_then(|a, (b, c, d)| handlers::run(a, b, c, d))
}

fn with_ops<C>(
    con: C,
    ops: impl GraphStoreOps<C>,
) -> impl Filter<Extract = ((C, impl GraphStoreOps<C>),), Error = std::convert::Infallible> + Clone
where
    C: StoreConnection,
{
    warp::any().map(move || (con.clone(), ops.clone()))
}

fn with_ops_and_tx<C>(
    con: C,
    ops: impl GraphStoreOps<C>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<
    Extract = ((C, impl GraphStoreOps<C>, Outgoing<Request, ServiceError>),),
    Error = std::convert::Infallible,
> + Clone
where
    C: StoreConnection,
{
    warp::any().map(move || (con.clone(), ops.clone(), tx.clone()))
}

use crate::common::body;
use crate::graph::handlers;
use crate::graph::models::GraphEntity;
use crate::graph::store::GraphStore;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::supervisor::Request;
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for graphs api.
pub fn graphs<T>(
    store: impl GraphStore<T>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    graphs_list(store.clone())
        .or(graphs_find(store.clone()))
        .or(graphs_put(store.clone()))
        .or(graphs_delete(store.clone()))
        .or(graphs_run(store.clone(), tx))
}

pub fn graphs_list<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs")
        .and(warp::get())
        .and(with_ops2(store))
        .and_then(handlers::list)
}

pub fn graphs_find<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs" / String)
        .and(warp::get())
        .and(with_ops2(store))
        .and_then(|a, b| handlers::find(a, b))
}

pub fn graphs_put<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs" / String)
        .and(warp::put())
        .and(body::json::<GraphEntity>())
        .and(with_ops2(store))
        .and_then(|a, b, c| handlers::put(a, b, c))
}

pub fn graphs_delete<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs" / String)
        .and(warp::delete())
        .and(with_ops2(store))
        .and_then(|a, b| handlers::delete(a, b))
}

pub fn graphs_run<T>(
    store: impl GraphStore<T>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs" / String / "run")
        .and(warp::post())
        .and(with_ops_and_tx2(store, tx))
        .and_then(|a, (b, c)| handlers::run(a, b, c))
}

fn with_ops2<T>(
    ops: impl GraphStore<T>,
) -> impl Filter<Extract = (impl GraphStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || ops.clone())
}

fn with_ops_and_tx2<T>(
    ops: impl GraphStore<T>,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<
    Extract = ((impl GraphStore<T>, Outgoing<Request, ServiceError>),),
    Error = std::convert::Infallible,
> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || (ops.clone(), tx.clone()))
}

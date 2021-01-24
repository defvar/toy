use crate::common::body;
use crate::graph::handlers;
use crate::graph::store::GraphStore;
use toy_api::graph::GraphEntity;
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for graphs api.
pub fn graphs<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    graphs_list(store.clone())
        .or(graphs_find(store.clone()))
        .or(graphs_put(store.clone()))
        .or(graphs_delete(store.clone()))
    // .or(graphs_run(store.clone()))
}

pub fn graphs_list<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs")
        .and(warp::get())
        .and(with_store(store))
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
        .and(with_store(store))
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
        .and(with_store(store))
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
        .and(with_store(store))
        .and_then(|a, b| handlers::delete(a, b))
}

fn with_store<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = (impl GraphStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || store.clone())
}

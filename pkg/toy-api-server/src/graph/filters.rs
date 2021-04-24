use crate::common;
use crate::graph::handlers;
use crate::graph::store::GraphStore;
use toy_api::graph::{FindOption, ListOption, PutOption};
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
}

pub fn graphs_list<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs")
        .and(warp::get())
        .and(common::query::query_opt::<ListOption>())
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
        .and(common::query::query_opt::<FindOption>())
        .and(with_store(store))
        .and_then(handlers::find)
}

pub fn graphs_put<T>(
    store: impl GraphStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("graphs" / String)
        .and(warp::put())
        .and(common::query::query_opt::<PutOption>())
        .and(common::body::bytes())
        .and(with_store(store))
        .and_then(handlers::put)
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

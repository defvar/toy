use crate::common::{self, body};
use crate::services::handlers;
use crate::services::store::ServiceStore;
use toy_api::services::{FindOption, ListOption, PutOption};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for services api.
pub fn services<T>(
    store: impl ServiceStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    find(store.clone())
        .or(list(store.clone()))
        .or(put(store.clone()))
        .or(delete(store.clone()))
}

fn list<T>(
    store: impl ServiceStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("services")
        .and(warp::get())
        .and(with_store(store))
        .and(common::query::query_opt::<ListOption>())
        .and_then(handlers::list)
}

fn find<T>(
    store: impl ServiceStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("services" / String)
        .and(warp::get())
        .and(with_store(store))
        .and(common::query::query_opt::<FindOption>())
        .and_then(|a, b, c| handlers::find(a, b, c))
}

fn put<T>(
    store: impl ServiceStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("services" / String)
        .and(warp::put())
        .and(common::query::query_opt::<PutOption>())
        .and(body::bytes())
        .and(with_store(store))
        .and_then(handlers::put)
}

fn delete<T>(
    store: impl ServiceStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path!("services" / String)
        .and(warp::delete())
        .and(with_store(store))
        .and_then(|a, b| handlers::delete(a, b))
}

fn with_store<T>(
    store: impl ServiceStore<T>,
) -> impl Filter<Extract = (impl ServiceStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || store.clone())
}

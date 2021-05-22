use crate::authentication::Auth;
use crate::common;
use crate::store::kv::KvStore;
use toy_api::services::{ServiceSpec, ServiceSpecList};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for services api.
pub fn services<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    crate::find!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone(),
        |v: ServiceSpec| v
    )
    .or(crate::list!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone(),
        |v: Vec<ServiceSpec>| ServiceSpecList::new(v)
    ))
    .or(crate::put!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone(),
        |v: ServiceSpec| Ok(v)
    ))
    .or(crate::delete!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone()
    ))
}

use crate::authentication::Auth;
use crate::common;
use crate::common::validator::{OkValidator, Validator};
use crate::store::kv::KvStore;
use toy_api::graph::{Graph, GraphList};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for graphs api.
pub fn graphs<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    crate::find!(
        warp::path("graphs"),
        auth.clone(),
        client.clone(),
        common::constants::GRAPHS_KEY_PREFIX,
        store.clone(),
        |v: Graph| v
    )
    .or(crate::list!(
        warp::path("graphs"),
        auth.clone(),
        client.clone(),
        common::constants::GRAPHS_KEY_PREFIX,
        store.clone(),
        |v: Vec<Graph>| GraphList::new(v)
    ))
    .or(crate::put!(
        warp::path("graphs"),
        auth.clone(),
        client.clone(),
        common::constants::GRAPHS_KEY_PREFIX,
        store.clone(),
        OkValidator::<Graph>::validate
    ))
    .or(crate::delete!(
        warp::path("graphs"),
        auth.clone(),
        client.clone(),
        common::constants::GRAPHS_KEY_PREFIX,
        store.clone()
    ))
}

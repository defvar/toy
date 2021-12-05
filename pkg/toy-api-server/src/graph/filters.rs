use crate::authentication::Auth;
use crate::common;
use crate::common::validator::{OkValidator, Validator};
use crate::store::kv::{KvStore, ListOption};
use crate::warp::filters::BoxedFilter;
use toy_api::graph::{Graph, GraphList, GraphListOption};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for graphs api.
pub fn graphs<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    store: impl KvStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    crate::find!(
        warp::path("graphs"),
        auth.clone(),
        client.clone(),
        common::constants::GRAPHS_KEY_PREFIX,
        store.clone(),
        |v: Graph| v
    )
    .or(crate::list_with_opt!(
        warp::path("graphs"),
        auth.clone(),
        client.clone(),
        common::constants::GRAPHS_KEY_PREFIX,
        store.clone(),
        GraphListOption,
        |_: Option<&GraphListOption>| ListOption::new(),
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
    .boxed()
}

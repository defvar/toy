use crate::authentication::Auth;
use crate::common;
use crate::common::validator::{OkValidator, Validator};
use crate::store::kv::{KvStore, ListOption};
use crate::warp::filters::BoxedFilter;
use toy_api::services::{ServiceSpec, ServiceSpecList, ServiceSpecListOption};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for services api.
pub fn services<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    store: impl KvStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    crate::find!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone(),
        |v: ServiceSpec| v
    )
    .or(crate::list_with_opt!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone(),
        ServiceSpecListOption,
        |_: Option<&ServiceSpecListOption>| ListOption::new(),
        |v: Vec<ServiceSpec>| ServiceSpecList::new(v)
    ))
    .or(crate::put!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone(),
        OkValidator::<ServiceSpec>::validate
    ))
    .or(crate::delete!(
        warp::path("services"),
        auth.clone(),
        client.clone(),
        common::constants::SERVICES_KEY_PREFIX,
        store.clone()
    ))
    .boxed()
}

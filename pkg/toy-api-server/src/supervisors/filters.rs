use crate::authentication::Auth;
use crate::common;
use crate::common::validator::OkValidator;
use crate::store::kv::{KvStore, ListOption};
use crate::warp::filters::BoxedFilter;
use toy_api::supervisors::{Supervisor, SupervisorList, SupervisorListOption};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for supervisors api.
pub fn supervisors<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    store: impl KvStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    crate::find!(
        warp::path("supervisors"),
        auth.clone(),
        client.clone(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        store.clone(),
        |v: Supervisor| v
    )
    .or(crate::list_with_opt!(
        warp::path("supervisors"),
        auth.clone(),
        client.clone(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        store.clone(),
        SupervisorListOption,
        |_: Option<&SupervisorListOption>| ListOption::new(),
        |v: Vec<Supervisor>| SupervisorList::new(v)
    ))
    .or(crate::put!(
        warp::path("supervisors"),
        auth.clone(),
        client.clone(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        store.clone(),
        OkValidator::<Supervisor>::new()
    ))
    .or(crate::delete!(
        warp::path("supervisors"),
        auth.clone(),
        client.clone(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        store.clone()
    ))
    .boxed()
}

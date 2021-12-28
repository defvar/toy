use crate::authentication::{authenticate, Auth};
use crate::common;
use crate::common::validator::{OkValidator, Validator};
use crate::store::kv::{KvStore, ListOption};
use crate::supervisors::handlers;
use crate::warp::filters::BoxedFilter;
use toy_api::supervisors::{Supervisor, SupervisorList, SupervisorListOption, SupervisorStatus};
use toy_h::HttpClient;
use warp::Filter;

macro_rules! transition_any {
    ($status_path: expr, $auth: expr, $client: expr, $store: expr, $status: expr) => {
        warp::path!("supervisors" / String / $status_path)
            .and(warp::put())
            .and(authenticate($auth, $client))
            .and(with_store($store))
            .and_then(|a, b, c| handlers::transition(a, b, c, $status))
    };
}

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
        OkValidator::<Supervisor>::validate
    ))
    .or(crate::delete!(
        warp::path("supervisors"),
        auth.clone(),
        client.clone(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        store.clone()
    ))
    .or(transition_any!(
        "stop",
        auth.clone(),
        client.clone(),
        store.clone(),
        SupervisorStatus::Stop
    ))
    .boxed()
}

fn with_store<T>(
    store: impl KvStore<T> + 'static,
) -> impl Filter<Extract = (impl KvStore<T> + 'static,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || store.clone())
}

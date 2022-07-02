use crate::authentication::{authenticate, Auth};
use crate::common;
use crate::common::validator::OkValidator;
use crate::store::kv::{KvStore, ListOption};
use crate::supervisors::handlers;
use crate::warp::filters::BoxedFilter;
use toy_api::common::PostOption;
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
    supervisors_beat(auth.clone(), client.clone(), store.clone())
        .or(crate::find!(
            warp::path("supervisors"),
            auth.clone(),
            client.clone(),
            common::constants::SUPERVISORS_KEY_PREFIX,
            store.clone(),
            |v: Supervisor| v
        ))
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

fn supervisors_beat<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    store: impl KvStore<T> + 'static,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>
where
    T: HttpClient + 'static,
{
    warp::path!("supervisors" / String / "beat")
        .and(warp::post())
        .and(authenticate(auth, client))
        .and(toy_api_http_common::query::query_opt::<PostOption>())
        .and(with_store(store))
        .and_then(handlers::beat)
}

fn with_store<T>(
    store: impl KvStore<T>,
) -> impl Filter<Extract = (impl KvStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || store.clone())
}

use crate::common::{self, body};
use crate::store::kv;
use crate::store::kv::KvStore;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_api::role::{Role, RoleList};
use toy_h::HttpClient;
use warp::Filter;

static ROLE: &'static str = "rbac/roles";
static ROLE_BINDING: &'static str = "rbac/roleBindings";

/// warp filter for rbac api.
pub fn rbac<T>(
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    common_operation(ROLE, common::constants::ROLE_KEY_PREFIX, store.clone()).or(common_operation(
        ROLE_BINDING,
        common::constants::ROLE_BINDING_KEY_PREFIX,
        store.clone(),
    ))
}

fn common_operation<T>(
    path: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    find(path, key_prefix, store.clone())
        .or(list(path, key_prefix, store.clone()))
        .or(put(path, key_prefix, store.clone()))
        .or(delete(path, key_prefix, store.clone()))
}

fn find<T>(
    path: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path(path)
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_store(store))
        .and(common::query::query_opt::<FindOption>())
        .and_then(move |key, store, opt| {
            common::handler::find(
                store,
                common::constants::generate_key(key_prefix, key),
                opt,
                kv::FindOption::new(),
                |v: Role| v,
            )
        })
}

fn list<T>(
    path: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path(path)
        .and(warp::get())
        .and(with_store(store))
        .and(common::query::query_opt::<ListOption>())
        .and_then(move |store, opt| {
            common::handler::list(
                store,
                key_prefix,
                opt,
                kv::ListOption::new(),
                |v: Vec<Role>| RoleList::new(v),
            )
        })
}

fn put<T>(
    path: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path(path)
        .and(warp::path::param::<String>())
        .and(warp::put())
        .and(common::query::query_opt::<PutOption>())
        .and(body::bytes())
        .and(with_store(store))
        .and_then(move |key, opt, req, store| {
            common::handler::put(
                store,
                common::constants::generate_key(key_prefix, key),
                opt,
                kv::PutOption::new(),
                req,
                |v: Role| Ok(v), //validation..?
            )
        })
}

fn delete<T>(
    path: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    warp::path(path)
        .and(warp::path::param::<String>())
        .and(warp::delete())
        .and(common::query::query_opt::<DeleteOption>())
        .and(with_store(store))
        .and_then(move |key, opt, store| {
            common::handler::delete(
                store,
                common::constants::generate_key(key_prefix, key),
                opt,
                kv::DeleteOption::new(),
            )
        })
}

fn with_store<T>(
    store: impl KvStore<T>,
) -> impl Filter<Extract = (impl KvStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || store.clone())
}

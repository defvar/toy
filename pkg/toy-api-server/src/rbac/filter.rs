use crate::authentication::Auth;
use crate::common::{self};
use crate::store::kv::KvStore;
use toy_api::role::{Role, RoleList};
use toy_h::HttpClient;
use warp::Filter;

/// warp filter for rbac api.
pub fn rbac<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    common_operation(
        auth.clone(),
        client.clone(),
        "rbac",
        "roles",
        common::constants::ROLE_KEY_PREFIX,
        store.clone(),
    )
    .or(common_operation(
        auth.clone(),
        client.clone(),
        "rbac",
        "roleBindings",
        common::constants::ROLE_BINDING_KEY_PREFIX,
        store.clone(),
    ))
}

fn common_operation<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    path: &'static str,
    path2: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    T: HttpClient,
{
    crate::find!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone(),
        |v: Role| v
    )
    .or(crate::list!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone(),
        |v: Vec<Role>| RoleList::new(v)
    ))
    .or(crate::put!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone(),
        |v: Role| Ok(v)
    ))
    .or(crate::delete!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone()
    ))
}

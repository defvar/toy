use crate::authentication::Auth;
use crate::common::validator::{OkValidator, Validator};
use crate::common::{self};
use crate::rbac::validator::RoleValidator;
use crate::store::kv::KvStore;
use toy_api::role::{Role, RoleList};
use toy_api::role_binding::{RoleBinding, RoleBindingList};
use toy_h::HttpClient;
use warp::filters::BoxedFilter;
use warp::Filter;

/// warp filter for rbac api.
pub fn rbac<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    store: impl KvStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    role(
        auth.clone(),
        client.clone(),
        "rbac",
        "roles",
        common::constants::ROLE_KEY_PREFIX,
        store.clone(),
    )
    .or(role_binding(
        auth.clone(),
        client.clone(),
        "rbac",
        "roleBindings",
        common::constants::ROLE_BINDING_KEY_PREFIX,
        store.clone(),
    ))
    .boxed()
}

fn role<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    path: &'static str,
    path2: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
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
        RoleValidator::validate
    ))
    .or(crate::delete!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone()
    ))
    .boxed()
}

fn role_binding<T>(
    auth: impl Auth<T> + Clone + 'static,
    client: T,
    path: &'static str,
    path2: &'static str,
    key_prefix: &'static str,
    store: impl KvStore<T> + 'static,
) -> BoxedFilter<(impl warp::Reply,)>
where
    T: HttpClient + 'static,
{
    crate::find!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone(),
        |v: RoleBinding| v
    )
    .or(crate::list!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone(),
        |v: Vec<RoleBinding>| RoleBindingList::new(v)
    ))
    .or(crate::put!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone(),
        OkValidator::<RoleBinding>::validate
    ))
    .or(crate::delete!(
        warp::path(path).and(warp::path(path2)),
        auth.clone(),
        client.clone(),
        key_prefix,
        store.clone()
    ))
    .boxed()
}

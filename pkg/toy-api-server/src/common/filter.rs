use crate::store::kv::KvStore;
use toy_h::HttpClient;
use warp::Filter;

#[macro_export]
macro_rules! find {
    ($path: expr, $auth: expr, $client: expr, $key_prefix: expr, $store: expr, $f: expr) => {
        $path
            .and(warp::path::param::<String>())
            .and(warp::get())
            .and($crate::authentication::authenticate($auth, $client))
            .and($crate::common::filter::with_store($store))
            .and($crate::common::query::query_opt::<
                toy_api::common::FindOption,
            >())
            .and_then(move |key, ctx, store, opt| {
                $crate::common::handler::find(
                    ctx,
                    store,
                    $crate::common::constants::generate_key($key_prefix, key),
                    opt,
                    $crate::store::kv::FindOption::new(),
                    $f,
                )
            })
    };
}

#[macro_export]
macro_rules! list {
    ($path: expr, $auth: expr, $client: expr, $key_prefix: expr, $store: expr, $f: expr) => {
        $path
            .and(warp::path::end())
            .and(warp::get())
            .and($crate::authentication::authenticate($auth, $client))
            .and($crate::common::filter::with_store($store))
            .and($crate::common::query::query_opt::<
                toy_api::common::ListOption,
            >())
            .and_then(move |ctx, store, opt| {
                $crate::common::handler::list(
                    ctx,
                    store,
                    $key_prefix,
                    opt,
                    $crate::store::kv::ListOption::new(),
                    $f,
                )
            })
    };
}

#[macro_export]
macro_rules! put {
    ($path: expr, $auth: expr, $client: expr, $key_prefix: expr, $store: expr, $f: expr) => {
        $path
            .and(warp::path::param::<String>())
            .and(warp::put())
            .and($crate::authentication::authenticate($auth, $client))
            .and($crate::common::query::query_opt::<toy_api::common::PutOption>())
            .and($crate::common::body::bytes())
            .and($crate::common::filter::with_store($store))
            .and_then(move |key, ctx, opt, req, store| {
                $crate::common::handler::put(
                    ctx,
                    store,
                    $crate::common::constants::generate_key($key_prefix, key),
                    opt,
                    $crate::store::kv::PutOption::new(),
                    req,
                    $f,
                )
            })
    };
}

#[macro_export]
macro_rules! delete {
    ($path: expr, $auth: expr, $client: expr, $key_prefix: expr, $store: expr) => {
        $path
            .and(warp::path::param::<String>())
            .and(warp::delete())
            .and($crate::authentication::authenticate($auth, $client))
            .and($crate::common::query::query_opt::<
                toy_api::common::DeleteOption,
            >())
            .and($crate::common::filter::with_store($store))
            .and_then(move |key, ctx, opt, store| {
                $crate::common::handler::delete(
                    ctx,
                    store,
                    $crate::common::constants::generate_key($key_prefix, key),
                    opt,
                    $crate::store::kv::DeleteOption::new(),
                )
            })
    };
}

pub fn with_store<T>(
    store: impl KvStore<T>,
) -> impl Filter<Extract = (impl KvStore<T>,), Error = std::convert::Infallible> + Clone
where
    T: HttpClient,
{
    warp::any().map(move || store.clone())
}

use crate::context::Context;
use crate::store::error::StoreError;
use crate::store::kv;
use crate::store::kv::{Delete, DeleteResult, Find, KvStore, List, Put};
use crate::{common, ApiError};
use std::convert::Infallible;
use toy_api::common::{DeleteOption, FindOption, ListOption, PutOption};
use toy_h::{Bytes, HttpClient};
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn find<H, Store, V, R, F>(
    ctx: Context,
    store: Store,
    key: String,
    api_opt: Option<FindOption>,
    store_opt: kv::FindOption,
    f: F,
) -> Result<impl warp::Reply, warp::Rejection>
where
    Store: KvStore<H>,
    H: HttpClient,
    F: FnOnce(V) -> R,
    V: DeserializableOwned,
    R: Serializable,
{
    tracing::trace!("handle: {:?}", ctx);
    match store
        .ops()
        .find::<V>(store.con().unwrap(), key, store_opt)
        .await
    {
        Ok(v) => match v {
            Some(v) => {
                let format = api_opt.as_ref().map(|x| x.format()).unwrap_or(None);
                let pretty = api_opt.as_ref().map(|x| x.pretty()).unwrap_or(None);
                let r = f(v);
                Ok(common::reply::into_response(&r, format, pretty))
            }
            None => Err(warp::reject::not_found()),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}

pub async fn list<H, Store, V, R, F>(
    ctx: Context,
    store: Store,
    prefix: &str,
    api_opt: Option<ListOption>,
    store_opt: kv::ListOption,
    f: F,
) -> Result<impl warp::Reply, warp::Rejection>
where
    Store: KvStore<H>,
    H: HttpClient,
    F: FnOnce(Vec<V>) -> R,
    V: DeserializableOwned,
    R: Serializable,
{
    tracing::trace!("handle: {:?}", ctx);
    match store
        .ops()
        .list::<V>(store.con().unwrap(), prefix.to_owned(), store_opt)
        .await
    {
        Ok(v) => {
            let format = api_opt.as_ref().map(|x| x.format()).unwrap_or(None);
            let pretty = api_opt.as_ref().map(|x| x.pretty()).unwrap_or(None);
            let r = f(v);
            Ok(common::reply::into_response(&r, format, pretty))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn put<H, Store, Req, V, F>(
    ctx: Context,
    store: Store,
    key: String,
    opt: Option<PutOption>,
    store_opt: kv::PutOption,
    request: Bytes,
    f: F,
) -> Result<impl warp::Reply, warp::Rejection>
where
    Store: KvStore<H>,
    H: HttpClient,
    Req: DeserializableOwned + Send,
    V: Serializable + Send,
    F: FnOnce(Req) -> Result<V, ApiError>,
{
    tracing::trace!("handle: {:?}", ctx);
    let format = opt.map(|x| x.format()).unwrap_or(None);
    let v = common::body::decode::<_, Req>(request, format)?;
    let v = f(v)?;
    match store
        .ops()
        .put(store.con().unwrap(), key, v, store_opt)
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => match e {
            StoreError::AllreadyExists { .. } => Ok(StatusCode::CONFLICT),
            _ => {
                tracing::error!("error:{:?}", e);
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}

pub async fn delete<H, Store>(
    ctx: Context,
    store: Store,
    key: String,
    _api_opt: Option<DeleteOption>,
    store_opt: kv::DeleteOption,
) -> Result<impl warp::Reply, Infallible>
where
    Store: KvStore<H>,
    H: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);
    match store
        .ops()
        .delete(store.con().unwrap(), key, store_opt)
        .await
    {
        Ok(r) => match r {
            DeleteResult::Deleted => Ok(StatusCode::NO_CONTENT),
            DeleteResult::NotFound => Ok(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

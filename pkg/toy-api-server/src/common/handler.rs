use crate::common::validator::Validator;
use crate::context::Context;
use crate::store::error::StoreError;
use crate::store::kv;
use crate::store::kv::{Delete, DeleteResult, Find, KvStore, List, Put};
use crate::{common, ApiError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::Infallible;
use std::fmt::Debug;
use toy_api::common::{
    DeleteOption, FindOption, KVObject, ListOption, ListOptionLike, PutOption, SelectionCandidate,
};
use toy_api_http_common::{codec, reply};
use toy_h::{Bytes, HttpClient};
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
    V: DeserializeOwned,
    R: Serialize,
{
    tracing::debug!("handle: {:?}", ctx);
    match store
        .ops()
        .find::<V>(store.con().unwrap(), key, store_opt)
        .await
    {
        Ok(v) => match v {
            Some(v) => {
                let format = api_opt.as_ref().map(|x| x.format()).unwrap_or(None);
                let indent = api_opt.as_ref().map(|x| x.indent()).unwrap_or(None);
                let r = f(v.into_value());
                Ok(reply::into_response(&r, format, indent))
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
    V: DeserializeOwned,
    R: Serialize,
{
    tracing::debug!("handle: {:?}", ctx);
    match store
        .ops()
        .list::<V>(store.con().unwrap(), prefix.to_owned(), store_opt)
        .await
    {
        Ok(v) => {
            let format = api_opt.as_ref().map(|x| x.format()).unwrap_or(None);
            let indent = api_opt.as_ref().map(|x| x.indent()).unwrap_or(None);
            let r = f(v.into_iter().map(|x| x.into_value()).collect());
            Ok(reply::into_response(&r, format, indent))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn list_with_opt<H, Store, V, R, Opt, StoreOptF, F>(
    ctx: Context,
    store: Store,
    prefix: &str,
    api_opt: Option<Opt>,
    store_opt_f: StoreOptF,
    f: F,
) -> Result<impl warp::Reply, warp::Rejection>
where
    H: HttpClient,
    Store: KvStore<H>,
    V: DeserializeOwned + SelectionCandidate,
    R: Serialize,
    Opt: ListOptionLike + Debug,
    StoreOptF: FnOnce(Option<&Opt>) -> kv::ListOption,
    F: FnOnce(Vec<V>) -> R,
{
    tracing::debug!("handle: ctx:{:?}, opt:{:?}", ctx, api_opt);

    match store
        .ops()
        .list::<V>(
            store.con().unwrap(),
            prefix.to_owned(),
            store_opt_f(api_opt.as_ref()),
        )
        .await
    {
        Ok(mut vec) => {
            let selection = api_opt.as_ref().map(|x| x.selection());
            let (format, indent) = api_opt
                .as_ref()
                .map(|x| (x.common().format(), x.common().indent()))
                .unwrap_or((None, None));

            match selection {
                Some(ref s) if !s.preds().is_empty() => {
                    // filter
                    vec = vec
                        .into_iter()
                        .filter(|item| s.is_match(item.value()))
                        .collect();
                }
                _ => {}
            };

            let r = f(vec.into_iter().map(|x| x.into_value()).collect());

            Ok(reply::into_response(&r, format, indent))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn put<H, Store, Req, T>(
    ctx: Context,
    store: Store,
    prefix: &'static str,
    key: String,
    opt: Option<PutOption>,
    store_opt: kv::PutOption,
    request: Bytes,
    validator: T,
) -> Result<impl warp::Reply, ApiError>
where
    Store: KvStore<H>,
    H: HttpClient,
    Req: DeserializeOwned + Serialize + KVObject + Send,
    T: Validator<H, Store, Req>,
{
    tracing::debug!("handle: {:?}", ctx);
    let format = opt.map(|x| x.format()).unwrap_or(None);
    let v = codec::decode::<_, Req>(request, format)?;
    let key_of_data = KVObject::key(&v).to_owned();

    if key_of_data != key {
        return Err(ApiError::difference_key(&key, &key_of_data));
    }

    let v = validator.validate(&ctx, &store, v).await?;
    match store
        .ops()
        .put(
            store.con().unwrap(),
            common::constants::generate_key(prefix, key),
            v,
            store_opt,
        )
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => match e {
            StoreError::AllreadyExists { .. } => Ok(StatusCode::CONFLICT),
            _ => {
                tracing::error!("error:{:?}", e);
                Err(ApiError::store_operation_failed("internal error..."))
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
    tracing::debug!("handle: {:?}", ctx);
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

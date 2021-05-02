use crate::common;
use crate::services::store::ServiceStore;
use crate::store::error::StoreError;
use crate::store::kv::{self, Delete, DeleteResult, Find, List, Put};
use std::convert::Infallible;
use toy_api::services as api;
use toy_api::services::ServiceSpec;
use toy_h::{Bytes, HttpClient};
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<T>(
    store: impl ServiceStore<T>,
    opt: Option<api::ListOption>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    match store
        .ops()
        .list::<api::ServiceSpec>(
            store.con().unwrap(),
            common::constants::SERVICES_KEY_PREFIX.to_string(),
            kv::ListOption::new(),
        )
        .await
    {
        Ok(v) => {
            let format = opt.map(|x| x.format()).unwrap_or(None);
            let services = api::ServiceSpecList::new(v);
            Ok(common::reply::into_response(&services, format))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn find<T>(
    key: String,
    store: impl ServiceStore<T>,
    opt: Option<api::FindOption>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::service_key(key);
    match store
        .ops()
        .find::<api::ServiceSpec>(store.con().unwrap(), key, kv::FindOption::new())
        .await
    {
        Ok(v) => match v {
            Some(v) => {
                let format = opt.map(|x| x.format()).unwrap_or(None);
                Ok(common::reply::into_response(&v, format))
            }
            None => Ok(StatusCode::NOT_FOUND.into_response()),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn put<T>(
    key: String,
    opt: Option<api::PutOption>,
    request: Bytes,
    store: impl ServiceStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let format = opt.map(|x| x.format()).unwrap_or(None);
    let v = common::body::decode::<_, ServiceSpec>(request, format)?;
    //
    // validation...?
    //
    let key = common::constants::service_key(key);
    match store
        .ops()
        .put(
            store.con().unwrap(),
            key,
            v,
            kv::PutOption::new().with_create_only(),
        )
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

pub async fn delete<T>(
    key: String,
    store: impl ServiceStore<T>,
) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    let key = common::constants::service_key(key);
    match store
        .ops()
        .delete(store.con().unwrap(), key, kv::DeleteOption::new())
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

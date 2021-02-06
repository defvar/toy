use crate::common;
use crate::store::kv::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult,
};
use crate::supervisors::store::SupervisorStore;
use std::convert::Infallible;
use toy_api::supervisors::{
    FindOption as ApiFindOption, ListOption as ApiListOption, Supervisor, Supervisors,
};
use toy_h::HttpClient;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<T>(
    store: impl SupervisorStore<T>,
    opt: Option<ApiListOption>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    match store
        .ops()
        .list::<Supervisor>(
            store.con().unwrap(),
            common::constants::SUPERVISORS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => {
            let format = opt.map(|x| x.format()).unwrap_or(None);
            let supervisors = Supervisors::new(v);
            Ok(common::reply::into_response(&supervisors, format))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn find<T>(
    key: String,
    store: impl SupervisorStore<T>,
    opt: Option<ApiFindOption>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::supervisor_key(key);
    match store
        .ops()
        .find::<Supervisor>(store.con().unwrap(), key, FindOption::new())
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
    v: Supervisor,
    store: impl SupervisorStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::supervisor_key(key);
    //
    // validation...?
    //
    match store
        .ops()
        .put(store.con().unwrap(), key, v, PutOption::new())
        .await
    {
        Ok(r) => match r {
            PutResult::Create => Ok(StatusCode::CREATED),
            PutResult::Update => Ok(StatusCode::OK),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete<T>(
    key: String,
    store: impl SupervisorStore<T>,
) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    let key = common::constants::supervisor_key(key);
    match store
        .ops()
        .delete(store.con().unwrap(), key, DeleteOption::new())
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

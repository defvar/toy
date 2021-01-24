use super::store::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult,
};
use crate::common;
use crate::graph::store::GraphStore;
use std::convert::Infallible;
use toy_api::graph::GraphEntity;
use toy_h::HttpClient;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<T>(store: impl GraphStore<T>) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    match store
        .ops()
        .list(
            store.con().unwrap(),
            common::constants::GRAPHS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => Ok(common::reply::json(&v).into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn find<T>(
    key: String,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
    match store
        .ops()
        .find(store.con().unwrap(), key, FindOption::new())
        .await
    {
        Ok(v) => match v {
            Some(v) => Ok(common::reply::json(&v).into_response()),
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
    v: GraphEntity,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
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
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, Infallible>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
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

// pub async fn run<T>(
//     key: String,
//     store: impl GraphStore<T>,
// ) -> Result<impl warp::Reply, warp::Rejection>
// where
//     T: HttpClient,
// {
//     let key = common::constants::graph_key(key);
//     match store
//         .ops()
//         .find(store.con().unwrap(), key, FindOption::new())
//         .await
//     {
//         Ok(Some(v)) => {
//             let pending = PendingEntity::new(v);
//             let id = TaskId::new();
//             let key = common::constants::pending_key(id);
//             match store
//                 .ops()
//                 .pending(store.con().unwrap(), key, pending)
//                 .await
//             {
//                 Ok(()) => Ok(common::reply::json(&(PendingResult::from_id(id))).into_response()),
//                 Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
//             }
//         }
//         Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
//         Err(e) => {
//             tracing::error!("error:{:?}", e);
//             Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
//         }
//     }
// }

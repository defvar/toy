use crate::graph::models::GraphEntity;
use crate::store::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult, StoreConnection, StoreOpsFactory,
};
use crate::{common, ApiError};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    match ops
        .list(
            con,
            common::constants::GRAPHS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => {
            let r = v
                .into_iter()
                .try_fold(Vec::new(), |mut vec, x| {
                    let r = toy_pack_json::unpack::<GraphEntity>(&x);
                    match r {
                        Ok(entity) => {
                            vec.push(entity);
                            Ok(vec)
                        }
                        Err(e) => Err(e),
                    }
                })
                .map_err(|e| warp::reject::custom(ApiError::from(e)))?;
            Ok(common::reply::json(&r).into_response())
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn find<C>(
    key: String,
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = common::constants::graph_key(key);
    match ops.find(con, key, FindOption::new()).await {
        Ok(v) => match v {
            Some(v) => {
                let v = toy_pack_json::unpack::<GraphEntity>(&v)
                    .map_err(|e| warp::reject::custom(ApiError::from(e)))?;
                Ok(common::reply::json(&v).into_response())
            }
            None => Ok(StatusCode::NOT_FOUND.into_response()),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

pub async fn put<C>(
    key: String,
    v: GraphEntity,
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = common::constants::graph_key(key);
    //
    // validation...?
    //
    let v = toy_pack_json::pack(&v).map_err(|e| warp::reject::custom(ApiError::from(e)))?;
    match ops.put(con, key, v, PutOption::new()).await {
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

pub async fn delete<C>(
    key: String,
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> Result<impl warp::Reply, Infallible>
where
    C: StoreConnection,
{
    let ops = ops.create().unwrap();
    let key = common::constants::graph_key(key);
    match ops.delete(con, key, DeleteOption::new()).await {
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

use crate::graph::store::GraphStore;
use crate::store::kv::{
    Delete, DeleteOption, DeleteResult, Find, FindOption, List, ListOption, Put, PutOption,
    PutResult,
};
use crate::{common, ApiError};
use toy_api::graph::{
    FindOption as ApiFindOption, GraphEntity, GraphsEntity, ListOption as ApiListOption,
    PutOption as ApiPutOption,
};
use toy_h::{Bytes, HttpClient};
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list<T>(
    opt: Option<ApiListOption>,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let format = opt.map(|x| x.format()).unwrap_or(None);
    store
        .ops()
        .list::<GraphEntity>(
            store.con().unwrap(),
            common::constants::GRAPHS_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
        .map(|x| {
            let graphs = GraphsEntity::new(x);
            Ok(common::reply::into_response(&graphs, format))
        })
        .map_err(|e| ApiError::store_operation_failed(e).into_rejection())
}

pub async fn find<T>(
    key: String,
    opt: Option<ApiFindOption>,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let format = opt.map(|x| x.format()).unwrap_or(None);
    let key = common::constants::graph_key(key);
    store
        .ops()
        .find::<GraphEntity>(store.con().unwrap(), key, FindOption::new())
        .await
        .map(|x| match x {
            Some(x) => Ok(common::reply::into_response(&x, format)),
            None => Ok(StatusCode::NOT_FOUND.into_response()),
        })
        .map_err(|e| ApiError::store_operation_failed(e).into_rejection())
}

pub async fn put<T>(
    key: String,
    opt: Option<ApiPutOption>,
    request: Bytes,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let format = opt.map(|x| x.format()).unwrap_or(None);
    let v = common::body::decode::<_, GraphEntity>(request, format)?;

    //
    // validation...?
    //

    let key = common::constants::graph_key(key);
    store
        .ops()
        .put::<GraphEntity>(store.con().unwrap(), key, v, PutOption::new())
        .await
        .map(|x| match x {
            PutResult::Create => Ok(StatusCode::CREATED),
            PutResult::Update => Ok(StatusCode::OK),
        })
        .map_err(|e| ApiError::store_operation_failed(e).into_rejection())
}

pub async fn delete<T>(
    key: String,
    store: impl GraphStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    let key = common::constants::graph_key(key);
    store
        .ops()
        .delete(store.con().unwrap(), key, DeleteOption::new())
        .await
        .map(|x| match x {
            DeleteResult::Deleted => Ok(StatusCode::NO_CONTENT),
            DeleteResult::NotFound => Ok(StatusCode::NOT_FOUND),
        })
        .map_err(|e| ApiError::store_operation_failed(e).into_rejection())
}

use crate::common::error::ApiError;
use crate::graph::handlers;
use crate::store::{StoreConnection, StoreOpsFactory};
use log;
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::Value;
use toy_core::supervisor::Request;
use warp::Filter;

pub fn graphs<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    graphs_list(con.clone(), ops.clone())
        .or(graphs_exec(con.clone(), tx.clone()))
        .or(graphs_find(con.clone(), ops.clone()))
        .or(graphs_put(con.clone(), ops.clone()))
        .or(graphs_delete(con.clone(), ops.clone()))
}

pub fn graphs_list<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs")
        .and(warp::get())
        .and(with_ops(store, ops))
        .and_then(|(a, b)| handlers::list(a, b))
}

pub fn graphs_find<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::get())
        .and(with_ops(store, ops))
        .and_then(|a, (b, c)| handlers::find(a, b, c))
}

pub fn graphs_exec(
    store: impl StoreConnection,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs" / String / "exec")
        .and(warp::get())
        .and(with_graphs(store, tx))
        .and_then(|a, (b, c)| handlers::exec(a, b, c))
}

pub fn graphs_put<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_ops(store, ops))
        .and_then(|a, b, (c, d)| handlers::put(a, b, c, d))
}

pub fn graphs_delete<C>(
    store: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    C: StoreConnection,
{
    warp::path!("graphs" / String)
        .and(warp::delete())
        .and(with_ops(store, ops))
        .and_then(|a, (b, c)| handlers::delete(a, b, c))
}

fn with_graphs(
    store: impl StoreConnection,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<
    Extract = ((impl StoreConnection, Outgoing<Request, ServiceError>),),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || (store.clone(), tx.clone()))
}

fn with_ops<C>(
    con: C,
    ops: impl StoreOpsFactory<C> + Clone,
) -> impl Filter<Extract = ((C, impl StoreOpsFactory<C> + Clone),), Error = std::convert::Infallible>
       + Clone
where
    C: StoreConnection,
{
    warp::any().map(move || (con.clone(), ops.clone()))
}

fn yaml_body() -> impl Filter<Extract = (Value,), Error = warp::Rejection> + Clone {
    // warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::body::aggregate().and_then(|buf| async move {
        let s = buf_to_string(buf);
        match s {
            Ok(x) => toy_pack_yaml::unpack::<Value>(x.as_str())
                .map_err(|e| warp::reject::custom(ApiError::from(e))),
            Err(e) => Err(warp::reject::custom(e)),
        }
    })
}

fn json_body() -> impl Filter<Extract = (Value,), Error = warp::Rejection> + Clone {
    // warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::body::aggregate().and_then(|buf| async move {
        let s = buf_to_string(buf);
        match s {
            Ok(x) => toy_pack_json::unpack::<Value>(x.as_bytes())
                .map_err(|e| warp::reject::custom(ApiError::from(e))),
            Err(e) => Err(warp::reject::custom(e)),
        }
    })
}

fn buf_to_string<T: warp::Buf>(buf: T) -> Result<String, ApiError> {
    std::str::from_utf8(buf.bytes())
        .map(|x| {
            log::debug!("receive:{:?}", x.to_string());
            x.to_string()
        })
        .map_err(|_| ApiError::error("body invalid utf8 sequence."))
}

use crate::common::error::ApiError;
use crate::graph::handlers;
use crate::persist::GraphRegistry;
use log;
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::prelude::Value;
use toy_core::supervisor::Request;
use warp::Filter;

pub fn graphs(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    graphs_list(g.clone(), tx.clone())
        .or(graphs_exec(g.clone(), tx.clone()))
        .or(graphs_find(g.clone(), tx.clone()))
        .or(graphs_put(g.clone(), tx.clone()))
        .or(graphs_delete(g, tx.clone()))
}

pub fn graphs_list(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs")
        .and(warp::get())
        .and(with_graphs(g, tx))
        .and_then(|(a, _)| handlers::list(a))
}

pub fn graphs_find(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs" / String)
        .and(warp::get())
        .and(with_graphs(g, tx))
        .and_then(|a, (b, _)| handlers::find(a, b))
}

pub fn graphs_exec(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs" / String / "exec")
        .and(warp::get())
        .and(with_graphs(g, tx))
        .and_then(|a, (b, c)| handlers::exec(a, b, c))
}

pub fn graphs_put(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs" / String)
        .and(warp::put())
        .and(yaml_body())
        .and(with_graphs(g, tx))
        .and_then(|_, b, (c, _)| handlers::put(b, c))
}

pub fn graphs_delete(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("graphs" / String)
        .and(warp::delete())
        .and(with_graphs(g, tx))
        .and_then(|a, (b, _)| handlers::delete(a, b))
}

fn with_graphs(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<
    Extract = ((GraphRegistry, Outgoing<Request, ServiceError>),),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || (g.clone(), tx.clone()))
}

fn yaml_body() -> impl Filter<Extract = (Value,), Error = warp::Rejection> + Clone {
    // warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::body::aggregate().and_then(|buf| {
        async move {
            let s = buf_to_string(buf);
            match s {
                Ok(x) => toy_pack_yaml::unpack::<Value>(x.as_str())
                    .map_err(|e| warp::reject::custom(ApiError::from(e))),
                Err(e) => Err(warp::reject::custom(e)),
            }
        }
    })
}

fn buf_to_string<T: warp::Buf>(buf: T) -> Result<String, ApiError> {
    std::str::from_utf8(buf.bytes())
        .map(|x| {
            log::info!("string----->{:?}", x.to_string());
            x.to_string()
        })
        .map_err(|_| ApiError::error("body invalid utf8 sequence."))
}

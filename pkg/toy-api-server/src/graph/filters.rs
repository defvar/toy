use crate::graph::handlers;
use crate::graph::GraphRegistry;
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::supervisor::Request;
use warp::Filter;

pub fn graphs(
    g: GraphRegistry,
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    graphs_list(g.clone(), tx.clone())
        .or(graphs_exec(g.clone(), tx.clone()))
        .or(graphs_find(g.clone(), tx.clone()))
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
    warp::path!("graphs" / "exec" / String)
        .and(warp::get())
        .and(with_graphs(g, tx))
        .and_then(|a, (b, c)| handlers::exec(a, b, c))
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

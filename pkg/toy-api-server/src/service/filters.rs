use crate::service::handlers;
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::supervisor::Request;
use warp::Filter;

pub fn services(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    service_list(tx.clone())
}

pub fn service_list(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("services")
        .and(warp::get())
        .and(with_tx(tx))
        .and_then(handlers::list)
}

fn with_tx(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = (Outgoing<Request, ServiceError>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || tx.clone())
}

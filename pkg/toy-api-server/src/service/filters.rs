use crate::common;
use crate::service::handlers;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::supervisor::Request;
use toy_api::service::ListOption;
use warp::Filter;

/// warp filter for services api.
pub fn services(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(tx.clone())
}

fn list(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("services")
        .and(warp::get())
        .and(with_tx(tx))
        .and(common::query::query_opt::<ListOption>())
        .and_then(handlers::list)
}

fn with_tx(
    tx: Outgoing<Request, ServiceError>,
) -> impl Filter<Extract = (Outgoing<Request, ServiceError>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || tx.clone())
}

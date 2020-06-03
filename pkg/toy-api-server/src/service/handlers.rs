use crate::common;
use std::convert::Infallible;
use toy_core::error::ServiceError;
use toy_core::mpsc::Outgoing;
use toy_core::oneshot;
use toy_core::supervisor::{Request, Response};
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list(mut tx: Outgoing<Request, ServiceError>) -> Result<impl warp::Reply, Infallible> {
    let (tx_, rx_) = oneshot::channel::<Response, ServiceError>();
    let _ = tx.send_ok(Request::Services(tx_)).await;
    if let Some(Ok(r)) = rx_.recv().await {
        match r {
            Response::Services(ref services) => Ok(common::reply::json(services).into_response()),
            // _ => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        }
    } else {
        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

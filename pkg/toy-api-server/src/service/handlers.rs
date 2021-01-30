use crate::common;
use std::convert::Infallible;
use toy::core::error::ServiceError;
use toy::core::mpsc::Outgoing;
use toy::core::oneshot;
use toy::supervisor::{Request, Response};
use toy_api::service::ListOption;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn list(
    mut tx: Outgoing<Request, ServiceError>,
    opt: Option<ListOption>,
) -> Result<impl warp::Reply, Infallible> {
    let (tx_, rx_) = oneshot::channel::<Response, ServiceError>();
    let _ = tx.send_ok(Request::Services(tx_)).await;
    if let Some(Ok(r)) = rx_.recv().await {
        match r {
            Response::Services(ref services) => {
                let format = opt.map(|x| x.format()).unwrap_or(None);
                Ok(common::reply::into_response(services, format))
            } // _ => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        }
    } else {
        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

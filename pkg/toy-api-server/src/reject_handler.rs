use crate::common;
use common::error::ApiError;
use std::convert::Infallible;
use toy_api::common::Format;
use toy_api::error::ErrorMessage;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = StatusCode::NOT_FOUND.to_string();
    } else if let Some(e) = err.find::<ApiError>() {
        code = e.status_code();
        message = e.error_message();
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::NOT_FOUND;
        message = StatusCode::NOT_FOUND.to_string();
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{}", e);
    } else {
        tracing::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = StatusCode::INTERNAL_SERVER_ERROR.to_string();
    }

    let r = common::reply::into_response(
        &ErrorMessage::new(code.as_u16(), message),
        Some(Format::Json),
        None,
    );
    Ok(warp::reply::with_status(r, code))
}

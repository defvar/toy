#[cfg(feature = "server_axum")]
use axum::response::{IntoResponse, Response};
use std::fmt::Display;
use thiserror::Error as ThisError;
use toy_api::error::ErrorMessage;
use toy_h::error::HError;
use toy_h::InvalidUri;
use toy_pack_urlencoded::QueryParseError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    SerializeValue {
        #[from]
        source: toy_core::data::error::SerializeError,
    },

    #[error(transparent)]
    DeserializeValue {
        #[from]
        source: toy_core::data::error::DeserializeError,
    },

    #[error(transparent)]
    DeserializeJsonValue {
        #[from]
        source: toy_pack_json::DecodeError,
    },

    #[error(transparent)]
    SerializeJsonValue {
        #[from]
        source: toy_pack_json::EncodeError,
    },

    #[error(transparent)]
    DeserializeMessagePackValue {
        #[from]
        source: toy_pack_mp::DecodeError,
    },

    #[error(transparent)]
    SerializeMessagePackValue {
        #[from]
        source: toy_pack_mp::EncodeError,
    },

    #[error(transparent)]
    InvalidUri {
        #[from]
        source: InvalidUri,
    },

    #[error(transparent)]
    QueryParse {
        #[from]
        source: QueryParseError,
    },

    #[error(transparent)]
    HError {
        #[from]
        source: HError,
    },

    #[error("code: {}, message: {}", inner.code(), inner.message())]
    ApiError { inner: ErrorMessage },

    #[error("{:?}", inner)]
    Error { inner: String },
}

impl Error {
    pub fn error<T>(msg: T) -> Error
    where
        T: Display,
    {
        Error::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<ErrorMessage> for Error {
    fn from(e: ErrorMessage) -> Self {
        Error::ApiError { inner: e }
    }
}

#[cfg(feature = "server")]
impl warp::reject::Reject for Error {}

#[cfg(feature = "server_axum")]
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let e = ErrorMessage::new(http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "");
        let json = toy_pack_json::pack_to_string(&e);
        match json {
            Ok(v) => (http::StatusCode::INTERNAL_SERVER_ERROR, v).into_response(),
            Err(_) => (http::StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response(),
        }
    }
}

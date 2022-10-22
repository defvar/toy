#[cfg(feature = "server")]
use crate::reply;
#[cfg(feature = "server")]
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use std::fmt::Display;
use thiserror::Error as ThisError;
use toy_api::common::Format;
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

    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::ApiError { inner } => {
                StatusCode::from_u16(inner.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            Error::ApiError { inner } => inner.message().to_string(),
            _ => self.to_string(),
        }
    }
}

impl From<ErrorMessage> for Error {
    fn from(e: ErrorMessage) -> Self {
        Error::ApiError { inner: e }
    }
}

#[cfg(feature = "server")]
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let e = ErrorMessage::new(self.status_code().as_u16(), self.error_message());
        let code = StatusCode::from_u16(e.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let r = reply::into_response(&e, Some(Format::Json), None);
        (code, r).into_response()
    }
}

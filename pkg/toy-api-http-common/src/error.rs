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

impl warp::reject::Reject for Error {}

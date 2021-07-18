use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;

use toy_api::error::ErrorMessage;
#[cfg(feature = "http")]
use toy_h::error::HError;
#[cfg(feature = "http")]
use toy_h::InvalidUri;
#[cfg(feature = "http")]
use toy_pack_urlencoded::QueryParseError;

#[derive(Debug, Error)]
pub enum ApiClientError {
    #[error("authentication failed. {:?}", inner)]
    AuthenticationFailed { inner: String },

    #[error("error: {:?}", source)]
    DeserializeJsonValue {
        #[from]
        source: toy_pack_json::DecodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    SerializeJsonValue {
        #[from]
        source: toy_pack_json::EncodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    DeserializeMessagePackValue {
        #[from]
        source: toy_pack_mp::DecodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    SerializeMessagePackValue {
        #[from]
        source: toy_pack_mp::EncodeError,
        backtrace: Backtrace,
    },

    #[cfg(feature = "http")]
    #[error("invalid uri: {:?}", source)]
    InvalidUri {
        #[from]
        source: InvalidUri,
        backtrace: Backtrace,
    },

    #[cfg(feature = "http")]
    #[error("failed http request: {:?}", source)]
    HError {
        #[from]
        source: HError,
    },

    #[cfg(feature = "http")]
    #[error("error: {:?}", source)]
    QueryParse {
        #[from]
        source: QueryParseError,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl ApiClientError {
    pub fn error<T>(msg: T) -> ApiClientError
    where
        T: Display,
    {
        ApiClientError::Error {
            inner: msg.to_string(),
        }
    }

    #[cfg(feature = "http")]
    pub fn query_parse(e: QueryParseError) -> ApiClientError {
        ApiClientError::QueryParse { source: e }
    }
}

impl From<ErrorMessage> for ApiClientError {
    fn from(e: ErrorMessage) -> Self {
        ApiClientError::error(format!("code:{}, message:{}", e.code(), e.message()))
    }
}

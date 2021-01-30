use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;
use toy_h::error::HError;
use toy_h::InvalidUri;
use toy_pack_json::{DecodeError, EncodeError};

#[derive(Debug, Error)]
pub enum ApiClientError {
    #[error("error: {:?}", source)]
    DeserializeJsonValue {
        #[from]
        source: DecodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    SerializeJsonValue {
        #[from]
        source: EncodeError,
        backtrace: Backtrace,
    },

    #[error("invalid uri: {:?}", source)]
    InvalidUri {
        #[from]
        source: InvalidUri,
        backtrace: Backtrace,
    },

    #[error("failed http request: {:?}", source)]
    HError {
        #[from]
        source: HError,
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
}

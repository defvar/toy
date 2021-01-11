use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;
use toy_glogging::auth::GAuthError;
use toy_glogging::error::GLoggingError;
use toy_pack_json::{DecodeError, EncodeError};

#[derive(Debug, Error)]
pub enum StoreGLoggingError {
    #[error("request error: {:?}", source)]
    Request {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[error("deserialize error: {:?}", source)]
    DeserializeJsonValue {
        #[from]
        source: DecodeError,
        backtrace: Backtrace,
    },

    #[error("serialize error: {:?}", source)]
    SerializeJsonValue {
        #[from]
        source: EncodeError,
        backtrace: Backtrace,
    },

    #[error("authentication failed. {:?}", source)]
    AuthenticationFailed {
        #[from]
        source: GAuthError,
        backtrace: Backtrace,
    },

    #[error("glogging api failed. {:?}", source)]
    GLoggingError {
        #[from]
        source: GLoggingError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl StoreGLoggingError {
    pub fn error<T>(msg: T) -> StoreGLoggingError
    where
        T: Display,
    {
        StoreGLoggingError::Error {
            inner: msg.to_string(),
        }
    }
}

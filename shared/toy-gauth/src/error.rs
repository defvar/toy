use crate::token::GTokenError;
use std::backtrace::Backtrace;
use std::fmt::Display;
use std::io;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum GAuthError {
    #[error("authentication failed. {:?}", inner)]
    AuthenticationFailed { inner: String },

    #[error("request token error. {:?}", inner)]
    RequestTokenError { inner: GTokenError },

    #[error("io error: {:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    DeserializeError {
        #[from]
        source: toy_pack_json::DecodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    SerializeError {
        #[from]
        source: toy_pack_json::EncodeError,
        backtrace: Backtrace,
    },

    #[error("io error: {:?}", source)]
    RequestError {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl GAuthError {
    pub fn error<T>(msg: T) -> GAuthError
    where
        T: Display,
    {
        GAuthError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn authentication_failed<T>(msg: T) -> GAuthError
    where
        T: Display,
    {
        GAuthError::AuthenticationFailed {
            inner: msg.to_string(),
        }
    }

    pub fn request_token_error(v: GTokenError) -> GAuthError {
        GAuthError::RequestTokenError { inner: v }
    }
}

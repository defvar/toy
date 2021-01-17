use crate::models::{ErrorInfo, ErrorResponse};
use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum GLoggingError {
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

    #[error("error: {:?}", source)]
    AuthenticationFailed {
        #[from]
        source: toy_gauth::error::GAuthError,
    },

    #[error("error: {:?}", inner)]
    GApiError { inner: Vec<ErrorInfo> },

    #[error("error: {:?}", source)]
    HError {
        #[from]
        source: toy_h::error::HError,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl GLoggingError {
    pub fn error<T>(msg: T) -> GLoggingError
    where
        T: Display,
    {
        GLoggingError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn api_error(inner: ErrorInfo) -> GLoggingError {
        GLoggingError::GApiError { inner: vec![inner] }
    }

    pub fn api_errors(inner: Vec<ErrorInfo>) -> GLoggingError {
        GLoggingError::GApiError { inner }
    }
}

impl From<ErrorResponse> for GLoggingError {
    fn from(e: ErrorResponse) -> Self {
        GLoggingError::GApiError {
            inner: vec![e.into_error_info()],
        }
    }
}

impl From<Vec<ErrorResponse>> for GLoggingError {
    fn from(e: Vec<ErrorResponse>) -> Self {
        let errors = e
            .into_iter()
            .map(|x| x.into_error_info())
            .collect::<Vec<_>>();
        GLoggingError::GApiError { inner: errors }
    }
}

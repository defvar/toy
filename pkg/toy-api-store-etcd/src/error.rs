use std::backtrace::Backtrace;
use std::fmt::Display;
use std::num::ParseIntError;
use thiserror::Error;
use toy_api_server::store::error::StoreErrorCustom;

#[derive(Debug, Error)]
pub enum EtcdError {
    #[error("{source}")]
    InvalidBase64String {
        #[from]
        source: base64::DecodeError,
        backtrace: Backtrace,
    },

    #[error("{source}")]
    InvalidUTF8String {
        #[from]
        source: std::str::Utf8Error,
        backtrace: Backtrace,
    },

    #[error("invalid version string: {source}")]
    InvalidVersionString {
        #[from]
        source: ParseIntError,
        backtrace: Backtrace,
    },

    #[error("error: {inner}")]
    Error { inner: String },
}

impl EtcdError {
    pub fn error<T>(msg: T) -> EtcdError
    where
        T: Display,
    {
        EtcdError::Error {
            inner: msg.to_string(),
        }
    }
}

impl StoreErrorCustom for EtcdError {}

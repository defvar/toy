use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;
use toy_pack_json::DecodeError;

#[derive(Debug, Error)]
pub enum StoreEtcdError {
    #[error("error: {:?}", source)]
    IO {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    Request {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    InvalidBase64String {
        #[from]
        source: base64::DecodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
    DeserializeJsonValue { inner: DecodeError },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl StoreEtcdError {
    pub fn error<T>(msg: T) -> StoreEtcdError
    where
        T: Display,
    {
        StoreEtcdError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<DecodeError> for StoreEtcdError {
    fn from(e: DecodeError) -> StoreEtcdError {
        StoreEtcdError::DeserializeJsonValue { inner: e }
    }
}

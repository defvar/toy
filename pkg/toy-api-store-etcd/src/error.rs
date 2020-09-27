use std::backtrace::Backtrace;
use std::fmt::Display;
use std::num::ParseIntError;
use thiserror::Error;
use toy_pack_json::{DecodeError, EncodeError};

#[derive(Debug, Error)]
pub enum StoreEtcdError {
    #[error("io error: {:?}", source)]
    IO {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("request error: {:?}", source)]
    Request {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[error("invalid base64 string: {:?}", source)]
    InvalidBase64String {
        #[from]
        source: base64::DecodeError,
        backtrace: Backtrace,
    },

    #[error("invalid version string: {:?}", source)]
    InvalidVersionString {
        #[from]
        source: ParseIntError,
        backtrace: Backtrace,
    },

    #[error("deserialize error: {:?}", source)]
    DeserializeJsonValue {
        #[from]
        source: DecodeError,
    },

    #[error("serialize error: {:?}", source)]
    SerializeJsonValue {
        #[from]
        source: EncodeError,
    },

    #[error("expected one result, but multiple. key:{:?}", key)]
    MultipleResult { key: String },

    #[error("entity not found. key:{:?}", key)]
    NotFound { key: String },

    #[error("failed operation. operation: {:?} key:{:?}", ops, key)]
    FailedOperation { ops: String, key: String },

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

    pub fn multiple_result<T>(key: T) -> StoreEtcdError
    where
        T: Display,
    {
        StoreEtcdError::MultipleResult {
            key: key.to_string(),
        }
    }

    pub fn not_found<T>(key: T) -> StoreEtcdError
    where
        T: Display,
    {
        StoreEtcdError::NotFound {
            key: key.to_string(),
        }
    }

    pub fn failed_opration<O, T>(ops: O, key: T) -> StoreEtcdError
    where
        O: Display,
        T: Display,
    {
        StoreEtcdError::FailedOperation {
            ops: ops.to_string(),
            key: key.to_string(),
        }
    }
}

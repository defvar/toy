use std::backtrace::Backtrace;
use std::fmt::Display;
use std::num::ParseIntError;
use thiserror::Error;
use toy_api_server::store::error::StoreErrorCustom;
use toy_h::error::HError;
use toy_h::http::uri::InvalidUri;
use toy_pack_json::{DecodeError, EncodeError};

#[derive(Debug, Error)]
pub enum StoreEtcdError {
    #[error("io error: {:?}", source)]
    IO {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("invalid base64 string: {:?}", source)]
    InvalidBase64String {
        #[from]
        source: base64::DecodeError,
        backtrace: Backtrace,
    },

    #[error("invalid utf-8 string: {:?}", source)]
    InvalidUTF8String {
        #[from]
        source: std::str::Utf8Error,
        backtrace: Backtrace,
    },

    #[error("invalid version string: {:?}", source)]
    InvalidVersionString {
        #[from]
        source: ParseIntError,
        backtrace: Backtrace,
    },

    #[error("invalid uri: {:?}", source)]
    InvalidUri {
        #[from]
        source: InvalidUri,
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

    #[error("failed operation. operation: {:?} key:{:?}", ops, key)]
    FailedOperation { ops: String, key: String },

    #[error("failed http request: {:?}", source)]
    HError {
        #[from]
        source: HError,
    },

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

impl StoreErrorCustom for StoreEtcdError {}

//! Error returned from the Store.

use std::backtrace::Backtrace;
use std::fmt::{Debug, Display};
use thiserror::Error;
use toy_h::error::HError;
use toy_h::http::uri::InvalidUri;
use toy_pack_json::{DecodeError, EncodeError};

/// A marker trait to ensure proper types are used for custom error.
pub trait StoreErrorCustom: Debug + Sized + Send + Sync + 'static {}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("store initialize error: {:?}", inner)]
    InitializeError { inner: String },

    #[error("error: {}", source)]
    IO {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("json deserialize error: {}", source)]
    DeserializeJsonValue {
        #[from]
        source: DecodeError,
    },

    #[error("json deserialize error. key:{}, error:{}", key, source)]
    DeserializeJsonValueWithKey { key: String, source: DecodeError },

    #[error("json serialize error: {}", source)]
    SerializeJsonValue {
        #[from]
        source: EncodeError,
    },

    #[error("deserialize error: {}", source)]
    DeserializeValue {
        #[from]
        source: toy_core::data::error::DeserializeError,
    },

    #[error("serialize error: {}", source)]
    SerializeValue {
        #[from]
        source: toy_core::data::error::SerializeError,
    },

    #[error("expected one result, but multiple. key:{:?}", key)]
    MultipleResult { key: String },

    #[error("key allready exists. key:{:?}", key)]
    AllreadyExists { key: String },

    #[error("entity not found. key:{:?}", key)]
    NotFoundUpdateTarget { key: String },

    #[error(
        "failed operation. operation: {:?}, key:{:?}, message:{:?}",
        ops,
        key,
        msg
    )]
    FailedOperation {
        ops: String,
        key: String,
        msg: String,
    },

    #[error("invalid uri: {}", source)]
    InvalidUri {
        #[from]
        source: InvalidUri,
        backtrace: Backtrace,
    },

    #[error("failed http request: {}", source)]
    HError {
        #[from]
        source: HError,
    },

    #[error("error: {}", inner)]
    Error { inner: String },
}

impl StoreError {
    pub fn error<T>(msg: T) -> StoreError
    where
        T: Display,
    {
        StoreError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn multiple_result<T>(key: T) -> StoreError
    where
        T: Display,
    {
        StoreError::MultipleResult {
            key: key.to_string(),
        }
    }

    pub fn allready_exists<T>(key: T) -> StoreError
    where
        T: Display,
    {
        StoreError::AllreadyExists {
            key: key.to_string(),
        }
    }

    pub fn not_found_update_target<T>(key: T) -> StoreError
    where
        T: Display,
    {
        StoreError::NotFoundUpdateTarget {
            key: key.to_string(),
        }
    }

    pub fn failed_opration<O, T, M>(ops: O, key: T, message: M) -> StoreError
    where
        O: Display,
        T: Display,
        M: Display,
    {
        StoreError::FailedOperation {
            ops: ops.to_string(),
            key: key.to_string(),
            msg: message.to_string(),
        }
    }

    pub fn deserialize_json_value_with_key(key: impl Into<String>, e: DecodeError) -> StoreError {
        StoreError::DeserializeJsonValueWithKey {
            key: key.into(),
            source: e,
        }
    }
}

fn custom<T>(err: T) -> StoreError
where
    T: StoreErrorCustom,
{
    StoreError::Error {
        inner: format!("{:?}", err),
    }
}

impl<T: StoreErrorCustom> From<T> for StoreError {
    #[inline]
    fn from(err: T) -> StoreError {
        custom(err)
    }
}

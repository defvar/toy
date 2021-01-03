//! Error returned from the Serialize or Deserialize.
//!

use crate::data::Value;
use std::fmt::Display;
use thiserror::Error as ThisError;
use toy_pack::{deser, ser};

#[derive(Debug, ThisError)]
pub enum SerializeError {
    #[error(
        "serialize error:invalid type. expected:{:?} but candidate:{:?}.",
        expected,
        candidate
    )]
    InvalidType { expected: String, candidate: Value },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

#[derive(Debug, ThisError)]
pub enum DeserializeError {
    #[error(
        "deserialize error:invalid type. expected:{:?} but candidate:{:?}.",
        expected,
        candidate
    )]
    InvalidType { expected: String, candidate: Value },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl SerializeError {
    pub fn error<T>(msg: T) -> SerializeError
    where
        T: Display,
    {
        SerializeError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_type<T>(expected: T, v: Value) -> SerializeError
    where
        T: Display,
    {
        SerializeError::InvalidType {
            expected: expected.to_string(),
            candidate: v,
        }
    }
}

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SerializeError::error(msg)
    }
}

impl DeserializeError {
    pub fn error<T>(msg: T) -> DeserializeError
    where
        T: Display,
    {
        DeserializeError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_type<T>(expected: T, v: Value) -> DeserializeError
    where
        T: Display,
    {
        DeserializeError::InvalidType {
            expected: expected.to_string(),
            candidate: v,
        }
    }
}

impl deser::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeserializeError::error(msg)
    }
}

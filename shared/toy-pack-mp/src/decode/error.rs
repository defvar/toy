use std::backtrace::Backtrace;
use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use thiserror::Error as ThisError;

use crate::marker::Marker;

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, ThisError)]
pub enum DecodeError {
    #[error("invalid type. decoded marker:{:?}, expected:{:?}", inner, expected)]
    InvalidType {
        inner: Marker,
        expected: String,
        backtrace: Backtrace,
    },

    #[error(
        "deserialize struct, must be a map type or array type. decoded marker: {:?}",
        inner
    )]
    InvalidStructType { inner: Marker, backtrace: Backtrace },

    #[error("num value out of range.")]
    OutOfRange,

    #[error("invalid utf8 sequence {:?}", source)]
    Utf8Error {
        #[from]
        source: Utf8Error,
        backtrace: Backtrace,
    },

    #[error("io error:{:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("{:?}", inner)]
    Error { inner: String },
}

impl DecodeError {
    pub fn error<T>(msg: T) -> DecodeError
    where
        T: Display,
    {
        DecodeError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_type<T>(marker: Marker, expected: T) -> DecodeError
    where
        T: Display,
    {
        DecodeError::InvalidType {
            inner: marker,
            expected: expected.to_string(),
            backtrace: Backtrace::capture(),
        }
    }

    pub fn invalid_struct_type(marker: Marker) -> DecodeError {
        DecodeError::InvalidStructType {
            inner: marker,
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<Marker> for DecodeError {
    fn from(marker: Marker) -> DecodeError {
        DecodeError::InvalidType {
            inner: marker,
            expected: "".to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl serde::de::Error for DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DecodeError::error(msg)
    }
}

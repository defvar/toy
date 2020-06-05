use std::backtrace::Backtrace;
use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use thiserror::Error as ThisError;
use toy_pack::deser::Error;

use crate::marker::Marker;

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, ThisError)]
pub enum DecodeError {
    #[error("invalid type. decoded marker:{:?}", inner)]
    InvalidType { inner: Marker },

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
}

impl From<Marker> for DecodeError {
    fn from(marker: Marker) -> DecodeError {
        DecodeError::InvalidType { inner: marker }
    }
}

impl Error for DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DecodeError::error(msg)
    }
}

use std::fmt::Display;
use std::io;

use toy_pack::ser::Error;

use crate::marker::Marker;

/// Using Encoder and Serializer.
/// It is used when an error occurs in the implementation of serialization.
///
#[derive(Debug, Fail)]
pub enum EncodeError {
    #[fail(display = "invalid type: {:?}", inner)]
    InvalidType { inner: Marker },

    #[fail(display = "io error:{:?}", inner)]
    IOError { inner: io::Error },

    #[fail(display = "{:?}", inner)]
    Error { inner: String },

    #[fail(display = "unknown seq length")]
    UnknownSeqLength,
}

impl EncodeError {
    pub fn error<T>(msg: T) -> EncodeError
    where
        T: Display,
    {
        EncodeError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn unknown_seq_length() -> EncodeError {
        EncodeError::UnknownSeqLength
    }
}

impl From<io::Error> for EncodeError {
    fn from(e: io::Error) -> EncodeError {
        EncodeError::IOError { inner: e }
    }
}

impl Error for EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        EncodeError::error(msg)
    }
}

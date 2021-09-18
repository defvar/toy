use std::backtrace::Backtrace;
use std::fmt::Display;
use std::io;
use thiserror::Error as ThisError;

/// Using Encoder and Serializer.
/// It is used when an error occurs in the implementation of serialization.
///
#[derive(Debug, ThisError)]
pub enum EncodeError {
    #[error("io error: {:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("{:?}", inner)]
    Error { inner: String },

    #[error("unknown seq length")]
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

impl serde::ser::Error for EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        EncodeError::error(msg)
    }
}

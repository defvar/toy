use std::backtrace::Backtrace;
use std::fmt::Display;
use std::io;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TailError {
    #[error("error: {:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    NotifyError {
        #[from]
        source: notify::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl TailError {
    pub fn error<T>(msg: T) -> TailError
    where
        T: Display,
    {
        TailError::Error {
            inner: msg.to_string(),
        }
    }
}

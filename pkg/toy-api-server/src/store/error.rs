//! Error returned from the Store.

use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("store initialize error: {:?}", inner)]
    InitializeError { inner: String },

    #[error("error: {:?}", source)]
    IO {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
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
}

impl warp::reject::Reject for StoreError {}

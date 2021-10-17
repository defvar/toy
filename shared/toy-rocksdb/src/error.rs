use rocksdb::Error;
use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RocksError {
    #[error("rocksdb error: {:?}", message)]
    Error {
        message: String,
        backtrace: Backtrace,
    },
}

impl RocksError {
    pub fn error<T>(msg: T) -> RocksError
    where
        T: Display,
    {
        RocksError::Error {
            message: msg.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<rocksdb::Error> for RocksError {
    fn from(e: Error) -> Self {
        RocksError::Error {
            message: e.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

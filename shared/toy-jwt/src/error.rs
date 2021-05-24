use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum JWTError {
    #[error("error: {:?}", inner)]
    Error { inner: String, backtrace: Backtrace },
}

impl JWTError {
    pub fn error<T>(msg: T) -> JWTError
    where
        T: Display,
    {
        JWTError::Error {
            inner: msg.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

use std::fmt::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum GAuthError {
    #[error("authentication failed. {:?}", inner)]
    AuthenticationFailed { inner: String },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl GAuthError {
    pub fn error<T>(msg: T) -> GAuthError
    where
        T: Display,
    {
        GAuthError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn authentication_failed<T>(msg: T) -> GAuthError
    where
        T: Display,
    {
        GAuthError::AuthenticationFailed {
            inner: msg.to_string(),
        }
    }
}

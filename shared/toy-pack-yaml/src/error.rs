use std::fmt::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum YamlError {
    #[error("yaml error: {:?}", inner)]
    Error { inner: String },
}

impl YamlError {
    pub fn error<T>(msg: T) -> YamlError
    where
        T: Display,
    {
        YamlError::Error {
            inner: msg.to_string(),
        }
    }
}

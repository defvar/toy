use std::fmt::Display;
use thiserror::Error as ThisError;
use toy_pack::schema;

/// Error parsing Struct to create Json Schema.
#[derive(Debug, ThisError)]
pub enum SchemaScanError {
    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl SchemaScanError {
    pub fn error<T>(msg: T) -> SchemaScanError
    where
        T: Display,
    {
        SchemaScanError::Error {
            inner: msg.to_string(),
        }
    }
}

impl schema::Error for SchemaScanError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SchemaScanError::error(msg)
    }
}

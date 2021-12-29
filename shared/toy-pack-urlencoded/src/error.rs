//! Error returned from the Deserializer.

use thiserror::Error;

/// Using Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
#[derive(Debug, Error)]
pub struct QueryParseError {
    err: Box<str>,
}

impl std::fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl QueryParseError {
    pub fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        QueryParseError {
            err: msg.to_string().into_boxed_str(),
        }
    }
}

use crate::data::Value;
use failure::Fail;
use std::fmt::Display;
use toy_pack::deser::Error;

#[derive(Debug, Fail)]
pub enum DeserializeError {
    #[fail(
        display = "deserialize error:invalid type. expected:{:?} but candidate:{:?}.",
        expected, candidate
    )]
    InvalidType { expected: String, candidate: Value },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

impl DeserializeError {
    pub fn error<T>(msg: T) -> DeserializeError
    where
        T: Display,
    {
        DeserializeError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_type<T>(expected: T, v: Value) -> DeserializeError
    where
        T: Display,
    {
        DeserializeError::InvalidType {
            expected: expected.to_string(),
            candidate: v,
        }
    }
}

impl Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeserializeError::error(msg)
    }
}

//! Error returned from the Deserializer.

use std::str::Utf8Error;
use thiserror::Error;

/// Using Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
#[derive(Debug, Error)]
pub struct QueryParseError {
    err: Box<str>,
}

impl QueryParseError {
    pub fn map_type_only() -> Self {
        QueryParseError {
            err: "deserialize or serialize, struct or map type only."
                .to_string()
                .into_boxed_str(),
        }
    }

    pub fn no_key() -> Self {
        QueryParseError {
            err: "tried to serialize a value before serializing key."
                .to_string()
                .into_boxed_str(),
        }
    }

    pub fn unsupported_key(tp: &'static str) -> Self {
        QueryParseError {
            err: format!("unsupported key, type: {}.", tp)
                .to_string()
                .into_boxed_str(),
        }
    }

    pub fn unsupported_value(tp: &'static str) -> Self {
        QueryParseError {
            err: format!("unsupported value, type: {}.", tp)
                .to_string()
                .into_boxed_str(),
        }
    }

    pub fn unsupported(tp: &'static str) -> Self {
        QueryParseError {
            err: format!("unsupported key or value, type: {}.", tp)
                .to_string()
                .into_boxed_str(),
        }
    }

    pub fn utf8_error(e: Utf8Error) -> Self {
        QueryParseError {
            err: format!("{:?}", e).to_string().into_boxed_str(),
        }
    }
}

impl std::fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.err)
    }
}

impl toy_pack::deser::Error for QueryParseError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        QueryParseError {
            err: msg.to_string().into_boxed_str(),
        }
    }
}

impl toy_pack::ser::Error for QueryParseError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        QueryParseError {
            err: msg.to_string().into_boxed_str(),
        }
    }
}

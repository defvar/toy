use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use thiserror::Error as ThisError;
use toy_pack::deser::Error;

use crate::decode::{Position, Token};
use std::backtrace::Backtrace;

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, ThisError)]
pub enum DecodeError {
    #[error("unexpected token:{:?}, expected:{:?}", unexpected, expected)]
    InvalidToken { expected: String, unexpected: Token },

    #[error("invalid utf8 sequence {:?}", source)]
    Utf8Error {
        #[from]
        source: Utf8Error,
        backtrace: Backtrace,
    },

    #[error("io error: {:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("invalid number")]
    InvalidNumber,

    #[error("{:?}", inner)]
    Error { inner: String },

    #[error("eof while parsing value")]
    EofWhileParsingValue,

    #[error("object key must be a String at line {} column {}", line, column)]
    KeyMustBeAString { line: usize, column: usize },

    #[error("expected comma or array end at line {} column {}", line, column)]
    ExpectedArrayCommaOrEnd { line: usize, column: usize },

    #[error("expected comma or object end at line {} column {}", line, column)]
    ExpectedObjectCommaOrEnd { line: usize, column: usize },

    #[error("trailing comma at line {} column {}.", line, column)]
    TrailingComma { line: usize, column: usize },

    #[error("expected colon at line {} column {}.", line, column)]
    ExpectedColon { line: usize, column: usize },
}

impl DecodeError {
    pub fn invalid_token<T>(unexpected: Token, expected: T) -> DecodeError
    where
        T: Display,
    {
        DecodeError::InvalidToken {
            unexpected,
            expected: expected.to_string(),
        }
    }

    pub fn error<T>(msg: T) -> DecodeError
    where
        T: Display,
    {
        DecodeError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_number() -> DecodeError {
        DecodeError::InvalidNumber
    }

    pub fn eof_while_parsing_value() -> DecodeError {
        DecodeError::EofWhileParsingValue
    }

    pub fn key_must_be_a_string(pos: Position) -> DecodeError {
        DecodeError::KeyMustBeAString {
            line: pos.line,
            column: pos.column,
        }
    }

    pub fn array_comma_or_end(pos: Position) -> DecodeError {
        DecodeError::ExpectedArrayCommaOrEnd {
            line: pos.line,
            column: pos.column,
        }
    }

    pub fn object_comma_or_end(pos: Position) -> DecodeError {
        DecodeError::ExpectedObjectCommaOrEnd {
            line: pos.line,
            column: pos.column,
        }
    }

    pub fn trailing_comma(pos: Position) -> DecodeError {
        DecodeError::TrailingComma {
            line: pos.line,
            column: pos.column,
        }
    }

    pub fn expected_colon(pos: Position) -> DecodeError {
        DecodeError::ExpectedColon {
            line: pos.line,
            column: pos.column,
        }
    }
}

impl Error for DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DecodeError::error(msg)
    }
}

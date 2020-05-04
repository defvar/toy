use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::io;
use std::str::Utf8Error;

use toy_pack::deser::Error;

use crate::decode::{Position, Token};

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum DecodeErrorKind {
    #[fail(display = "unexpected token:{:?}, expected:{:?}", unexpected, expected)]
    InvalidToken { expected: String, unexpected: Token },

    #[fail(display = "invalid utf8 sequence {:?}", inner)]
    Utf8Error { inner: Utf8Error },

    #[fail(display = "io error:{:?}. msg:{:?}", kind, msg)]
    IOError { kind: io::ErrorKind, msg: String },

    #[fail(display = "invalid number")]
    InvalidNumber,

    #[fail(display = "{:?}", inner)]
    Error { inner: String },

    #[fail(display = "eof while parsing value")]
    EofWhileParsingValue,

    #[fail(
        display = "object key must be a String at line {} column {}",
        line, column
    )]
    KeyMustBeAString { line: usize, column: usize },

    #[fail(
        display = "expected comma or array end at line {} column {}",
        line, column
    )]
    ExpectedArrayCommaOrEnd { line: usize, column: usize },

    #[fail(
        display = "expected comma or object end at line {} column {}",
        line, column
    )]
    ExpectedObjectCommaOrEnd { line: usize, column: usize },

    #[fail(display = "trailing comma at line {} column {}.", line, column)]
    TrailingComma { line: usize, column: usize },

    #[fail(display = "expected colon at line {} column {}.", line, column)]
    ExpectedColon { line: usize, column: usize },
}

#[derive(Debug)]
pub struct DecodeError {
    inner: Context<DecodeErrorKind>,
}

impl Fail for DecodeError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<DecodeErrorKind> for DecodeError {
    fn from(kind: DecodeErrorKind) -> DecodeError {
        DecodeError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<DecodeErrorKind>> for DecodeError {
    fn from(inner: Context<DecodeErrorKind>) -> DecodeError {
        DecodeError { inner }
    }
}

impl DecodeError {
    pub fn new(inner: Context<DecodeErrorKind>) -> DecodeError {
        DecodeError { inner }
    }

    pub fn kind(&self) -> &DecodeErrorKind {
        self.inner.get_context()
    }

    pub fn invalid_token<T>(unexpected: Token, expected: T) -> DecodeError
    where
        T: Display,
    {
        DecodeErrorKind::InvalidToken {
            unexpected,
            expected: expected.to_string(),
        }
        .into()
    }

    pub fn error<T>(msg: T) -> DecodeError
    where
        T: Display,
    {
        DecodeErrorKind::Error {
            inner: msg.to_string(),
        }
        .into()
    }

    pub fn invalid_number() -> DecodeError {
        DecodeErrorKind::InvalidNumber.into()
    }

    pub fn eof_while_parsing_value() -> DecodeError {
        DecodeErrorKind::EofWhileParsingValue.into()
    }

    pub fn key_must_be_a_string(pos: Position) -> DecodeError {
        DecodeErrorKind::KeyMustBeAString {
            line: pos.line,
            column: pos.column,
        }
        .into()
    }

    pub fn array_comma_or_end(pos: Position) -> DecodeError {
        DecodeErrorKind::ExpectedArrayCommaOrEnd {
            line: pos.line,
            column: pos.column,
        }
        .into()
    }

    pub fn object_comma_or_end(pos: Position) -> DecodeError {
        DecodeErrorKind::ExpectedObjectCommaOrEnd {
            line: pos.line,
            column: pos.column,
        }
        .into()
    }

    pub fn trailing_comma(pos: Position) -> DecodeError {
        DecodeErrorKind::TrailingComma {
            line: pos.line,
            column: pos.column,
        }
        .into()
    }

    pub fn expected_colon(pos: Position) -> DecodeError {
        DecodeErrorKind::ExpectedColon {
            line: pos.line,
            column: pos.column,
        }
        .into()
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(e: Utf8Error) -> DecodeError {
        DecodeErrorKind::Utf8Error { inner: e }.into()
    }
}

impl From<io::Error> for DecodeError {
    fn from(e: io::Error) -> DecodeError {
        DecodeErrorKind::IOError {
            kind: e.kind(),
            msg: format!("{}", e),
        }
        .into()
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

use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::io;
use std::str::Utf8Error;

use toy_pack::deser::Error;

use crate::decode::Token;

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, Fail)]
pub enum DecodeErrorKind {
    #[fail(display = "decode error:invalid type. decoded token:{:?}", inner)]
    InvalidType { inner: Token },

    #[fail(display = "decode error:num value out of range.")]
    OutOfRange,

    #[fail(display = "decode error:invalid utf8 sequence. sequence:{:?}", inner)]
    Utf8Error { inner: Utf8Error },

    #[fail(display = "decode error:io error:{:?}", inner)]
    IOError { inner: io::Error },

    #[fail(display = "decode error: invalid number")]
    InvalidNumber,

    #[fail(display = "decode error: {:?}", inner)]
    Error { inner: String },

    #[fail(display = "decode error: eof while parsing value")]
    EofWhileParsingValue,

    #[fail(display = "decode error: object key must be a String")]
    KeyMustBeAString,
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

    pub fn key_must_be_a_string() -> DecodeError {
        DecodeErrorKind::KeyMustBeAString.into()
    }
}

impl From<Token> for DecodeError {
    fn from(token: Token) -> DecodeError {
        DecodeErrorKind::InvalidType { inner: token }.into()
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(e: Utf8Error) -> DecodeError {
        DecodeErrorKind::Utf8Error { inner: e }.into()
    }
}

impl From<io::Error> for DecodeError {
    fn from(e: io::Error) -> DecodeError {
        DecodeErrorKind::IOError { inner: e }.into()
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

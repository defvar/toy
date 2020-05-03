use std::fmt::Display;
use std::io;
use std::str::Utf8Error;

use toy_pack::deser::Error;

use crate::decode::Token;

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, Fail)]
pub enum DecodeError {
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
}

impl DecodeError {
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
}

impl From<Token> for DecodeError {
    fn from(token: Token) -> DecodeError {
        DecodeError::InvalidType { inner: token }
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(e: Utf8Error) -> DecodeError {
        DecodeError::Utf8Error { inner: e }
    }
}

impl From<io::Error> for DecodeError {
    fn from(e: io::Error) -> DecodeError {
        DecodeError::IOError { inner: e }
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

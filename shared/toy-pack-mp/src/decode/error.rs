use std::fmt::Display;
use std::io;
use std::str::Utf8Error;

use toy_pack::deser::Error;

use crate::marker::Marker;

/// Using Decoder and Deserializer.
/// It is used when an error occurs in the implementation of deserialization.
///
#[derive(Debug, Fail)]
pub enum DecodeError {
    #[fail(display = "decode error:invalid type. decoded marker:{:?}", inner)]
    InvalidType {
        inner: Marker,
    },

    #[fail(display = "decode error:num value out of range.")]
    OutOfRange,

    #[fail(display = "decode error:invalid utf8 sequence. sequence:{:?}", inner)]
    Utf8Error {
        inner: Utf8Error,
    },

    #[fail(display = "decode error:io error:{:?}", inner)]
    IOError {
        inner: io::Error,
    },

    #[fail(display = "decode error: {:?}", inner)]
    Error {
        inner: String,
    },
}

impl DecodeError {
    pub fn error<T>(msg: T) -> DecodeError where T: Display {
        DecodeError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<Marker> for DecodeError {
    fn from(marker: Marker) -> DecodeError {
        DecodeError::InvalidType { inner: marker }
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
    fn custom<T>(msg: T) -> Self where T: Display {
        DecodeError::error(msg)
    }
}

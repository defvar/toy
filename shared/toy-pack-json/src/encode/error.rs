use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::io;

use toy_pack::ser::Error;

/// Using Encoder and Serializer.
/// It is used when an error occurs in the implementation of serialization.
///
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum EncodeErrorKind {
    #[fail(display = "invalid type: {:?}", inner)]
    InvalidType { inner: String },

    #[fail(display = "io error:{:?}. msg:{:?}", kind, msg)]
    IOError { kind: io::ErrorKind, msg: String },

    #[fail(display = "{:?}", inner)]
    Error { inner: String },
}

#[derive(Debug)]
pub struct EncodeError {
    inner: Context<EncodeErrorKind>,
}

impl Fail for EncodeError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<EncodeErrorKind> for EncodeError {
    fn from(kind: EncodeErrorKind) -> EncodeError {
        EncodeError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<EncodeErrorKind>> for EncodeError {
    fn from(inner: Context<EncodeErrorKind>) -> EncodeError {
        EncodeError { inner }
    }
}

impl EncodeError {
    pub fn kind(&self) -> &EncodeErrorKind {
        self.inner.get_context()
    }

    pub fn error<T>(msg: T) -> EncodeError
    where
        T: Display,
    {
        EncodeErrorKind::Error {
            inner: msg.to_string(),
        }
        .into()
    }
}

impl From<io::Error> for EncodeError {
    fn from(e: io::Error) -> EncodeError {
        EncodeErrorKind::IOError {
            kind: e.kind(),
            msg: format!("{}", e),
        }
        .into()
    }
}

impl Error for EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        EncodeError::error(msg)
    }
}

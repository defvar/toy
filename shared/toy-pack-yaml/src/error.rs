use std::fmt::{self, Display};

use failure::{Backtrace, Context, Fail};
use toy_pack::deser::Error as DeserError;
use toy_pack::ser::Error as SerError;

#[derive(Debug, Fail)]
pub enum YamlErrorKind {
    #[fail(display = "yaml error: {:?}", inner)]
    ScanError { inner: yaml_rust::ScanError },

    #[fail(display = "yaml error: {:?}", inner)]
    EmmitError { inner: yaml_rust::EmitError },

    #[fail(display = "yaml error: {:?}", inner)]
    Error { inner: String },
}

#[derive(Debug)]
pub struct YamlError {
    inner: Context<YamlErrorKind>,
}

impl Fail for YamlError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl YamlError {
    pub fn new(inner: Context<YamlErrorKind>) -> YamlError {
        YamlError { inner }
    }

    pub fn kind(&self) -> &YamlErrorKind {
        self.inner.get_context()
    }
}

impl From<YamlErrorKind> for YamlError {
    fn from(kind: YamlErrorKind) -> YamlError {
        YamlError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<YamlErrorKind>> for YamlError {
    fn from(inner: Context<YamlErrorKind>) -> YamlError {
        YamlError { inner }
    }
}

impl YamlError {
    pub fn error<T>(msg: T) -> YamlError
    where
        T: Display,
    {
        YamlErrorKind::Error {
            inner: msg.to_string(),
        }
        .into()
    }
}

impl From<yaml_rust::ScanError> for YamlError {
    fn from(e: yaml_rust::ScanError) -> Self {
        YamlErrorKind::ScanError { inner: e }.into()
    }
}

impl From<yaml_rust::EmitError> for YamlError {
    fn from(e: yaml_rust::EmitError) -> Self {
        YamlErrorKind::EmmitError { inner: e }.into()
    }
}

impl DeserError for YamlError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        YamlError::error(msg)
    }
}

impl SerError for YamlError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        YamlError::error(msg)
    }
}

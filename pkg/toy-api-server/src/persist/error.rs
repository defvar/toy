use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use toy_core::error::ConfigError;
use toy_pack_yaml::error::YamlError;

#[derive(Debug, Fail)]
pub enum PersistErrorKind {
    #[fail(display = "error: {:?}", inner)]
    IO { inner: std::io::Error },

    #[fail(display = "error: {:?}", inner)]
    DeserializeValue { inner: YamlError },

    #[fail(display = "error: {:?}", inner)]
    DeserializeConfig { inner: ConfigError },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

#[derive(Debug)]
pub struct PersistError {
    inner: Context<PersistErrorKind>,
}

impl Fail for PersistError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for PersistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl PersistError {
    pub fn new(inner: Context<PersistErrorKind>) -> PersistError {
        PersistError { inner }
    }

    pub fn kind(&self) -> &PersistErrorKind {
        self.inner.get_context()
    }

    pub fn error<T>(msg: T) -> PersistError
    where
        T: Display,
    {
        PersistErrorKind::Error {
            inner: msg.to_string(),
        }
        .into()
    }
}

impl From<PersistErrorKind> for PersistError {
    fn from(kind: PersistErrorKind) -> PersistError {
        PersistError {
            inner: Context::new(kind),
        }
    }
}

impl From<std::io::Error> for PersistError {
    fn from(e: std::io::Error) -> PersistError {
        PersistErrorKind::IO { inner: e }.into()
    }
}

impl From<YamlError> for PersistError {
    fn from(e: YamlError) -> PersistError {
        PersistErrorKind::DeserializeValue { inner: e }.into()
    }
}

impl From<ConfigError> for PersistError {
    fn from(e: ConfigError) -> PersistError {
        PersistErrorKind::DeserializeConfig { inner: e }.into()
    }
}

impl warp::reject::Reject for PersistError {}

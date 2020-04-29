use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use toy_core::error::ConfigError;
use toy_pack_yaml::error::YamlError;

#[derive(Debug, Fail)]
pub enum ApiErrorKind {
    #[fail(display = "error: {:?}", inner)]
    DeserializeValue { inner: YamlError },

    #[fail(display = "error: {:?}", inner)]
    DeserializeConfig { inner: ConfigError },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

#[derive(Debug)]
pub struct ApiError {
    inner: Context<ApiErrorKind>,
}

impl Fail for ApiError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ApiError {
    pub fn new(inner: Context<ApiErrorKind>) -> ApiError {
        ApiError { inner }
    }

    pub fn kind(&self) -> &ApiErrorKind {
        self.inner.get_context()
    }

    pub fn error<T>(msg: T) -> ApiError
    where
        T: Display,
    {
        ApiErrorKind::Error {
            inner: msg.to_string(),
        }
        .into()
    }
}

impl From<ApiErrorKind> for ApiError {
    fn from(kind: ApiErrorKind) -> ApiError {
        ApiError {
            inner: Context::new(kind),
        }
    }
}

impl From<YamlError> for ApiError {
    fn from(e: YamlError) -> ApiError {
        ApiErrorKind::DeserializeValue { inner: e }.into()
    }
}

impl From<ConfigError> for ApiError {
    fn from(e: ConfigError) -> ApiError {
        ApiErrorKind::DeserializeConfig { inner: e }.into()
    }
}

impl warp::reject::Reject for ApiError {}

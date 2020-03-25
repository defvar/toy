use std::fmt::Display;

use failure::Fail;
use futures::channel::mpsc::{SendError, TrySendError};
use futures::channel::oneshot;
use std::io;
use std::str::Utf8Error;

pub trait Error: Sized + Fail {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "channel canceled.")]
    ChannelCanceled,

    #[fail(display = "channel send error: {:?}", inner)]
    ChannelSendError { inner: SendError },

    #[fail(display = "config initialization failed. error: {:?}", inner)]
    ConfigInitFailed { inner: ConfigError },

    #[fail(display = "error: {:?}", inner)]
    IOError { inner: io::Error },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

impl ServiceError {
    pub fn error<T>(msg: T) -> ServiceError
    where
        T: Display,
    {
        ServiceError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<oneshot::Canceled> for ServiceError {
    fn from(_e: oneshot::Canceled) -> ServiceError {
        ServiceError::ChannelCanceled
    }
}

impl From<SendError> for ServiceError {
    fn from(e: SendError) -> Self {
        ServiceError::ChannelSendError { inner: e }
    }
}

impl<T> From<TrySendError<T>> for ServiceError {
    fn from(e: TrySendError<T>) -> Self {
        ServiceError::ChannelSendError {
            inner: e.into_send_error(),
        }
    }
}

impl From<ConfigError> for ServiceError {
    fn from(e: ConfigError) -> Self {
        ServiceError::ConfigInitFailed { inner: e }
    }
}

impl From<io::Error> for ServiceError {
    fn from(e: io::Error) -> Self {
        ServiceError::IOError { inner: e }
    }
}

impl Error for ServiceError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ServiceError::Error {
            inner: msg.to_string(),
        }
    }
}

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "config validation failed. error:{:?}", inner)]
    ValidationError { inner: String },

    #[fail(display = "invalid config. {:?}", inner)]
    Utf8Error { inner: Utf8Error },

    #[fail(display = "config load failed. {:?}", inner)]
    IOError { inner: io::Error },

    #[fail(display = "invalid config. not found key:{:?}", inner)]
    NotFoundKey { inner: String },

    #[fail(display = "invalid config key type. {:?} must be {:?}", key, expected)]
    InvalidKeyType { key: String, expected: String },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

impl ConfigError {
    pub fn error<T>(msg: T) -> ConfigError
    where
        T: Display,
    {
        ConfigError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn validation_error<T>(msg: T) -> ConfigError
    where
        T: Display,
    {
        ConfigError::ValidationError {
            inner: msg.to_string(),
        }
    }

    pub fn not_found_key<T>(key: T) -> ConfigError
    where
        T: Display,
    {
        ConfigError::NotFoundKey {
            inner: key.to_string(),
        }
    }

    pub fn invalid_key_type<T1, T2>(key: T1, expected: T2) -> ConfigError
    where
        T1: Display,
        T2: Display,
    {
        ConfigError::InvalidKeyType {
            key: key.to_string(),
            expected: expected.to_string(),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> ConfigError {
        ConfigError::IOError { inner: e }
    }
}

impl From<Utf8Error> for ConfigError {
    fn from(e: Utf8Error) -> ConfigError {
        ConfigError::Utf8Error { inner: e }
    }
}

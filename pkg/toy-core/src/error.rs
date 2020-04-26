use std::fmt::Display;

use crate::ServiceType;
use failure::Fail;
use std::io;
use std::str::Utf8Error;

pub trait Error: Sized + Fail {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "config initialization failed. error: {:?}", inner)]
    ConfigInitFailed { inner: ConfigError },

    #[fail(display = "error: {:?}", inner)]
    IOError { inner: io::Error },

    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },

    #[fail(display = "error: {:?}", inner)]
    ContextInitFailed { inner: String },

    #[fail(display = "not found service. service_type: {:?}", st)]
    ServiceNotFound { st: ServiceType },
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

    pub fn context_init_failed<T>(msg: T) -> ServiceError
    where
        T: Display,
    {
        ServiceError::ContextInitFailed {
            inner: msg.to_string(),
        }
    }

    pub fn service_not_found<T>(st: T) -> ServiceError
    where
        ServiceType: From<T>,
    {
        ServiceError::ServiceNotFound { st: From::from(st) }
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

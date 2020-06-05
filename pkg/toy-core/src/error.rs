use crate::ServiceType;
use std::backtrace::Backtrace;
use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use thiserror::Error as ThisError;

pub trait Error: Sized {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}

#[derive(Debug, ThisError)]
pub enum ServiceError {
    #[error("config initialization failed. error: {:?}", source)]
    ConfigInitFailed {
        #[from]
        source: ConfigError,
    },

    #[error("error: {:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },

    #[error("error: {:?}", inner)]
    ContextInitFailed { inner: String },

    #[error("not found service. service_type: {:?}", st)]
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

#[derive(Debug, ThisError)]
pub enum ConfigError {
    #[error("config validation failed. error:{:?}", inner)]
    ValidationError { inner: String },

    #[error("invalid config. {:?}", source)]
    Utf8Error {
        #[from]
        source: Utf8Error,
        backtrace: Backtrace,
    },

    #[error("config load failed. {:?}", source)]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("invalid config. not found key:{:?}", inner)]
    NotFoundKey { inner: String },

    #[error("invalid config key type. {:?} must be {:?}", key, expected)]
    InvalidKeyType { key: String, expected: String },

    #[error(
        "invalid service type. {:?} name_space:{:?} service_name:{:?}",
        msg,
        name_space,
        service_name
    )]
    InvalidServiceType {
        name_space: String,
        service_name: String,
        msg: String,
    },

    #[error("error: {:?}", inner)]
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

    pub fn invalid_service_type<P: Display, M: Display>(
        name_space: P,
        service_name: P,
        msg: M,
    ) -> ConfigError {
        ConfigError::InvalidServiceType {
            name_space: name_space.to_string(),
            service_name: service_name.to_string(),
            msg: msg.to_string(),
        }
    }
}

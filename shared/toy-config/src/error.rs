use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use toy_pack_yaml::error::YamlError;

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "config error:validation error:{:?}", inner)]
    ValidationError { inner: String },

    #[fail(display = "config error:invalid utf8 sequence. sequence:{:?}", inner)]
    Utf8Error { inner: Utf8Error },

    #[fail(display = "config error:io error:{:?}", inner)]
    IOError { inner: io::Error },

    #[fail(display = "config error:deserialization error:{:?}", inner)]
    DeserializationError { inner: YamlError },
}

impl ConfigError {
    pub fn validation_error<T>(msg: T) -> ConfigError
    where
        T: Display,
    {
        ConfigError::ValidationError { inner: msg.to_string() }
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

impl From<YamlError> for ConfigError {
    fn from(e: YamlError) -> ConfigError {
        ConfigError::DeserializationError { inner: e }
    }
}

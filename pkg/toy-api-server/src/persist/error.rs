use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error;
use toy_core::error::ConfigError;
use toy_pack_yaml::error::YamlError;

#[derive(Debug, Error)]
pub enum PersistError {
    #[error("error: {:?}", source)]
    IO {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", inner)]
    DeserializeValue { inner: YamlError },

    #[error("error: {:?}", source)]
    DeserializeConfig {
        #[from]
        source: ConfigError,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl PersistError {
    pub fn error<T>(msg: T) -> PersistError
    where
        T: Display,
    {
        PersistError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<YamlError> for PersistError {
    fn from(e: YamlError) -> PersistError {
        PersistError::DeserializeValue { inner: e }
    }
}

impl warp::reject::Reject for PersistError {}

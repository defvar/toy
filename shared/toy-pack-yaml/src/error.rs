use std::backtrace::Backtrace;
use std::fmt::Display;
use thiserror::Error as ThisError;
use toy_pack::deser::Error as DeserError;
use toy_pack::ser::Error as SerError;

#[derive(Debug, ThisError)]
pub enum YamlError {
    #[error("yaml error: {:?}", source)]
    ScanError {
        #[from]
        source: yaml_rust::ScanError,
        backtrace: Backtrace,
    },

    #[error("yaml error: {:?}", source)]
    EmmitError {
        #[from]
        source: yaml_rust::EmitError,
        backtrace: Backtrace,
    },

    #[error("yaml error: {:?}", inner)]
    Error { inner: String },
}

impl YamlError {
    pub fn error<T>(msg: T) -> YamlError
    where
        T: Display,
    {
        YamlError::Error {
            inner: msg.to_string(),
        }
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

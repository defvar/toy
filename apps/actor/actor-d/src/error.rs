use std::backtrace::Backtrace;
use std::fmt::Display;
use std::net::AddrParseError;
use thiserror::Error;
use toy_jwt::error::JWTError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("can not read credential file. {:?}", inner)]
    ReadCredentialError { inner: String },

    #[error("generate token failed. {:?}", source)]
    GenerateTokenFailed {
        #[from]
        source: JWTError,
    },

    #[error("not found env. {:?}", inner)]
    NotFoundEnv { inner: String },

    #[error("error: {:?}", source)]
    IOError {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error(transparent)]
    HError {
        #[from]
        source: toy::api_client::toy_h::error::HError,
    },

    #[error("error: {:?}", source)]
    JsonDeserializeError {
        #[from]
        source: toy_pack_json::DecodeError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    YamlDeserializeError {
        #[from]
        source: toy_pack_yaml::error::YamlError,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    GraphConfigError {
        #[from]
        source: toy::core::error::ConfigError,
        backtrace: Backtrace,
    },

    #[error("error: invalid log path")]
    InvalidLogPath,

    #[error("error: {:?}", source)]
    AddrParseError {
        #[from]
        source: AddrParseError,
        backtrace: Backtrace,
    },
}

impl Error {
    pub fn read_credential_error<T>(msg: T) -> Error
    where
        T: Display,
    {
        Error::ReadCredentialError {
            inner: msg.to_string(),
        }
    }

    pub fn not_found_env<T>(msg: T) -> Error
    where
        T: Display,
    {
        Error::NotFoundEnv {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_log_path() -> Error {
        Error::InvalidLogPath
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::not_found_env(e)
    }
}

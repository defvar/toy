use std::backtrace::Backtrace;
use std::fmt::Display;
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
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::not_found_env(e)
    }
}

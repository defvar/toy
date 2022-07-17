use std::fmt::Display;
use thiserror::Error;
use toy_jwt::error::JWTError;
use toy_pack_urlencoded::QueryParseError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("can not read credential file {}. Caused by:{}", path, cause)]
    ReadCredentialError { path: String, cause: String },

    #[error("generate token failed. {}", source)]
    GenerateTokenFailed {
        #[from]
        source: JWTError,
    },

    #[error(transparent)]
    ApiClientError {
        #[from]
        source: toy::api_client::error::ApiClientError,
    },

    #[error(transparent)]
    JsonSerializeError {
        #[from]
        source: toy_pack_json::EncodeError,
    },

    #[error(transparent)]
    JsonDeserializeError {
        #[from]
        source: toy_pack_json::DecodeError,
    },

    #[error("error: invalid file format. cause: {}", source)]
    InvalidJsonFormatError { source: toy_pack_json::DecodeError },

    #[error("not found env. {}", inner)]
    NotFoundEnv { inner: String },

    #[allow(dead_code)]
    #[error("unknwon resource. name: {}", name)]
    UnknwonResource { name: String },

    #[error(transparent)]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("error: invalid opt. cause: {}", source)]
    ParseOptionError {
        #[from]
        source: QueryParseError,
    },

    #[error("error: invalid log path.")]
    InvalidLogPath,
}

impl Error {
    pub fn read_credential_error(path: impl Into<String>, cause: Error) -> Error {
        Error::ReadCredentialError {
            path: path.into(),
            cause: cause.to_string(),
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

    #[allow(dead_code)]
    pub fn unknwon_resource<T>(name: T) -> Error
    where
        T: Display,
    {
        Error::UnknwonResource {
            name: name.to_string(),
        }
    }

    pub fn invalid_log_path() -> Error {
        Error::InvalidLogPath
    }

    pub fn invalid_file_format(err: toy_pack_json::DecodeError) -> Error {
        Error::InvalidJsonFormatError { source: err }
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::not_found_env(e)
    }
}

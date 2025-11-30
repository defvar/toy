use std::fmt::Display;
use thiserror::Error;
use toy_jwt::error::JWTError;
use toy_pack_urlencoded::QueryParseError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("can not read credential file {}. Caused by:{}", path, cause)]
    ReadCredential { path: String, cause: String },

    #[error("generate token failed. {}", source)]
    GenerateTokenFailed {
        #[from]
        source: JWTError,
    },

    #[error(transparent)]
    ApiClient {
        #[from]
        source: toy::api_client::error::ApiClientError,
    },

    #[error(transparent)]
    JsonSerialize {
        #[from]
        source: toy_pack_json::EncodeError,
    },

    #[error(transparent)]
    JsonDeserialize {
        #[from]
        source: toy_pack_json::DecodeError,
    },

    #[error("not found env. {}", inner)]
    NotFoundEnv { inner: String },

    #[allow(dead_code)]
    #[error("unknwon resource. name: {}", name)]
    UnknwonResource { name: String },

    #[error(transparent)]
    IO {
        #[from]
        source: std::io::Error,
    },

    #[error("error: invalid opt. cause: {}", source)]
    ParseOption {
        #[from]
        source: QueryParseError,
    },
}

impl Error {
    pub fn read_credential_error(path: impl Into<String>, cause: Error) -> Error {
        Error::ReadCredential {
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
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::not_found_env(e)
    }
}

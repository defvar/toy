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

    #[error("{:?}", source)]
    ApiClientError {
        #[from]
        source: toy::api_client::error::ApiClientError,
    },

    #[error("{:?}", source)]
    JsonSerializeError {
        #[from]
        source: toy_pack_json::EncodeError,
    },

    #[error("{:?}", source)]
    JsonDeserializeError {
        #[from]
        source: toy_pack_json::DecodeError,
    },

    #[error("not found env. {:?}", inner)]
    NotFoundEnv { inner: String },

    #[error("unknwon resource. name: {:?}", name)]
    UnknwonResource { name: String },
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

    pub fn unknwon_resource<T>(name: T) -> Error
    where
        T: Display,
    {
        Error::UnknwonResource {
            name: name.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::read_credential_error(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::not_found_env(e)
    }
}

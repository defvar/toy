use std::fmt::Display;
use thiserror::Error;
use toy_core::error::ConfigError;
use toy_pack_json::DecodeError;
use toy_pack_yaml::error::YamlError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("error: {:?}", inner)]
    DeserializeValue { inner: YamlError },

    #[error("error: {:?}", inner)]
    DeserializeJsonValue { inner: DecodeError },

    #[error("error: {:?}", inner)]
    DeserializeConfig { inner: ConfigError },

    #[error("authentication failed. {:?}", inner)]
    AuthenticationFailed { inner: String },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl ApiError {
    pub fn error<T>(msg: T) -> ApiError
    where
        T: Display,
    {
        ApiError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn authentication_failed<T>(msg: T) -> ApiError
    where
        T: Display,
    {
        ApiError::AuthenticationFailed {
            inner: msg.to_string(),
        }
    }
}

impl From<YamlError> for ApiError {
    fn from(e: YamlError) -> ApiError {
        ApiError::DeserializeValue { inner: e }
    }
}

impl From<DecodeError> for ApiError {
    fn from(e: DecodeError) -> ApiError {
        ApiError::DeserializeJsonValue { inner: e }
    }
}

impl From<ConfigError> for ApiError {
    fn from(e: ConfigError) -> ApiError {
        ApiError::DeserializeConfig { inner: e }
    }
}

impl warp::reject::Reject for ApiError {}

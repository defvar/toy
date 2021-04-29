use std::fmt::{Debug, Display};
use thiserror::Error;
use toy_core::error::ConfigError;
use toy_pack_json::{DecodeError, EncodeError};
use toy_pack_urlencoded::QueryParseError;
use toy_pack_yaml::error::YamlError;
use warp::http::StatusCode;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("error: {:?}", inner)]
    DeserializeValue { inner: YamlError },

    #[error("error: {:?}", inner)]
    DeserializeJsonValue { inner: DecodeError },

    #[error("error: {:?}", inner)]
    SerializeJsonValue { inner: EncodeError },

    #[error("error: {:?}", inner)]
    DeserializeMessagePackValue { inner: toy_pack_mp::DecodeError },

    #[error("error: {:?}", inner)]
    SerializeMessagePackValue { inner: toy_pack_mp::EncodeError },

    #[error("error: {:?}", inner)]
    DeserializeConfig { inner: ConfigError },

    #[error("authentication failed. {:?}", inner)]
    AuthenticationFailed { inner: String },

    #[error("error: {:?}", source)]
    QueryParse {
        #[from]
        source: QueryParseError,
    },

    #[error("store operation failed: {:?}", inner)]
    StoreOperationFailed { inner: String },

    #[error("task id invalid format: {:?}", id)]
    TaskIdInvalidFormat { id: String },

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

    pub fn status_code(&self) -> StatusCode {
        match self {
            //TODO: error code....
            ApiError::QueryParse { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> String {
        self.to_string()
    }

    pub fn authentication_failed<T>(msg: T) -> ApiError
    where
        T: Display,
    {
        ApiError::AuthenticationFailed {
            inner: msg.to_string(),
        }
    }

    pub fn query_parse(e: QueryParseError) -> ApiError {
        ApiError::QueryParse { source: e }
    }

    pub fn into_rejection(self) -> warp::Rejection {
        warp::reject::custom(self)
    }

    pub fn store_operation_failed<T>(msg: T) -> ApiError
    where
        T: Debug,
    {
        ApiError::StoreOperationFailed {
            inner: format!("{:?}", msg).to_string(),
        }
    }

    pub fn task_id_invalid_format(id: String) -> ApiError {
        ApiError::TaskIdInvalidFormat { id }
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

impl From<EncodeError> for ApiError {
    fn from(e: EncodeError) -> ApiError {
        ApiError::SerializeJsonValue { inner: e }
    }
}

impl From<toy_pack_mp::DecodeError> for ApiError {
    fn from(e: toy_pack_mp::DecodeError) -> ApiError {
        ApiError::DeserializeMessagePackValue { inner: e }
    }
}

impl From<toy_pack_mp::EncodeError> for ApiError {
    fn from(e: toy_pack_mp::EncodeError) -> ApiError {
        ApiError::SerializeMessagePackValue { inner: e }
    }
}

impl From<ConfigError> for ApiError {
    fn from(e: ConfigError) -> ApiError {
        ApiError::DeserializeConfig { inner: e }
    }
}

impl warp::reject::Reject for ApiError {}

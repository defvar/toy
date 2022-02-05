use std::fmt::{Debug, Display};
use thiserror::Error;
use toy_core::error::ConfigError;
use toy_pack_json::{DecodeError, EncodeError};
use toy_pack_urlencoded::QueryParseError;
use toy_pack_yaml::error::YamlError;
use warp::http::StatusCode;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    SerializeValue {
        #[from]
        source: toy_core::data::error::SerializeError,
    },

    #[error(transparent)]
    DeserializeValue {
        #[from]
        source: toy_core::data::error::DeserializeError,
    },

    #[error("{:?}", inner)]
    DeserializeYamlValue { inner: YamlError },

    #[error("{:?}", inner)]
    DeserializeJsonValue { inner: DecodeError },

    #[error("{:?}", inner)]
    SerializeJsonValue { inner: EncodeError },

    #[error("{:?}", inner)]
    DeserializeMessagePackValue { inner: toy_pack_mp::DecodeError },

    #[error("{:?}", inner)]
    SerializeMessagePackValue { inner: toy_pack_mp::EncodeError },

    #[error(transparent)]
    ParseGraphConfigFailed {
        #[from]
        source: ConfigError,
    },

    #[error("authentication failed. {:?}", inner)]
    AuthenticationFailed { inner: String },

    #[error(
        "authorization failed. user: {:?} cannot: {:?} resource: {:?}",
        user,
        verb,
        resource
    )]
    AuthorizationFailed {
        user: String,
        resource: String,
        verb: String,
    },

    #[error("{:?}", source)]
    QueryParse {
        #[from]
        source: QueryParseError,
    },

    #[error("store operation failed. {:?}", inner)]
    StoreOperationFailed { inner: String },

    #[error("task id invalid format. id:{:?}", id)]
    TaskIdInvalidFormat { id: String },

    #[error("server initialize failed. {:?}", inner)]
    ServerInitializeFailed { inner: String },

    #[error("validation failed. {}", inner)]
    ValidationFailed { inner: String },

    #[error("{:?}", inner)]
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
            ApiError::ValidationFailed { .. } => StatusCode::BAD_REQUEST,
            ApiError::QueryParse { .. } => StatusCode::BAD_REQUEST,
            ApiError::AuthorizationFailed { .. } => StatusCode::FORBIDDEN,
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

    pub fn authorization_failed<T: Into<String>>(user: T, resource: T, verb: T) -> ApiError {
        ApiError::AuthorizationFailed {
            user: user.into(),
            resource: resource.into(),
            verb: verb.into(),
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

    pub fn server_initialize_failed<T>(msg: T) -> ApiError
    where
        T: Display,
    {
        ApiError::ServerInitializeFailed {
            inner: msg.to_string(),
        }
    }

    pub fn validation_failed<T>(msg: T) -> ApiError
    where
        T: Display,
    {
        ApiError::ValidationFailed {
            inner: msg.to_string(),
        }
    }

    pub fn difference_key(specified_key: &str, key_of_data: &str) -> ApiError {
        ApiError::ValidationFailed {
            inner: format!("There is a difference between the specified key and the key of the object to be registered. specified:{}, data:{}", specified_key, key_of_data),
        }
    }
}

impl From<YamlError> for ApiError {
    fn from(e: YamlError) -> ApiError {
        ApiError::DeserializeYamlValue { inner: e }
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

impl warp::reject::Reject for ApiError {}

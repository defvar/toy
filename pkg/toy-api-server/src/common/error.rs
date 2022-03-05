use std::fmt::{Debug, Display};
use thiserror::Error;
use toy_api_http_common::Error;
use toy_core::error::ConfigError;
use toy_h::error::HError;
use toy_h::InvalidUri;
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

    #[error(transparent)]
    InvalidUri {
        #[from]
        source: InvalidUri,
    },

    #[error(transparent)]
    HError {
        #[from]
        source: HError,
    },

    #[error(transparent)]
    ApiHttpCommonError {
        #[from]
        source: toy_api_http_common::Error,
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
            ApiError::ValidationFailed { .. } => StatusCode::BAD_REQUEST,
            ApiError::ApiHttpCommonError { source } => match source {
                Error::QueryParse { .. } => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
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

impl warp::reject::Reject for ApiError {}

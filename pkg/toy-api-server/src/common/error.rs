use std::fmt::{Debug, Display};
use thiserror::Error;
use toy_api::common::Format;
use toy_api::error::ErrorMessage;
use toy_api_http_common::axum::response::{IntoResponse, Response};
use toy_api_http_common::reply;
use toy_core::error::ConfigError;
use toy_h::error::HError;
use toy_h::{InvalidUri, StatusCode};

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

    #[error("task id invalid format. id:{id}")]
    TaskIdInvalidFormat { id: String },

    #[error("server initialize failed. {inner}")]
    ServerInitializeFailed { inner: String },

    #[error("validation failed. {}", inner)]
    ValidationFailed { inner: String },

    #[error("allready exists. key:{}", key)]
    AllreadyExists { key: String },

    #[error("invalid selector {:?}", fields)]
    InvalidSelector { fields: Vec<String> },

    #[error("invalid field {field}")]
    InvalidField { field: String },

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
            ApiError::ApiHttpCommonError { source } => source.status_code(),
            ApiError::AllreadyExists { .. } => StatusCode::CONFLICT,
            ApiError::InvalidSelector { .. } => StatusCode::BAD_REQUEST,
            ApiError::InvalidField { .. } => StatusCode::BAD_REQUEST,
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

    pub fn allready_exists<T>(key: T) -> ApiError
    where
        T: Into<String>,
    {
        ApiError::AllreadyExists { key: key.into() }
    }

    pub fn difference_key(specified_key: &str, key_of_data: &str) -> ApiError {
        ApiError::ValidationFailed {
            inner: format!("There is a difference between the specified key and the key of the object to be registered. specified:{}, data:{}", specified_key, key_of_data),
        }
    }

    pub fn invalid_selectors(fields: Vec<String>) -> ApiError {
        ApiError::InvalidSelector { fields }
    }

    pub fn invalid_selector(field: String) -> ApiError {
        ApiError::InvalidSelector {
            fields: vec![field],
        }
    }

    pub fn invalid_field(field: impl Into<String>) -> ApiError {
        ApiError::InvalidField {
            field: field.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let e = ErrorMessage::new(self.status_code().as_u16(), self.error_message());
        let code = StatusCode::from_u16(e.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let r = reply::into_response(&e, Some(Format::Json), None);
        (code, r).into_response()
    }
}

use std::fmt::Display;
use thiserror::Error;
use toy_api::common::Format;
use toy_api::error::ErrorMessage;
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::Response;
use toy_api_http_common::reply;
use toy_core::error::ConfigError;

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error(transparent)]
    ApiHttpCommonError {
        #[from]
        source: toy_api_http_common::Error,
    },

    #[error(transparent)]
    ApiClientError {
        #[from]
        source: toy_api_client::error::ApiClientError,
    },

    #[error(transparent)]
    ParseGraphConfigFailed {
        #[from]
        source: ConfigError,
    },

    #[error("not found. key:{key}")]
    NotFound { key: String },

    #[error("task id invalid format. id:{id}")]
    TaskIdInvalidFormat { id: String },

    #[error("{:?}", inner)]
    Error { inner: String },
}

impl SupervisorError {
    pub fn task_id_invalid_format(id: String) -> SupervisorError {
        SupervisorError::TaskIdInvalidFormat { id }
    }

    pub fn not_found(key: impl Into<String>) -> SupervisorError {
        SupervisorError::NotFound { key: key.into() }
    }

    pub fn error<T>(msg: T) -> SupervisorError
    where
        T: Display,
    {
        SupervisorError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            SupervisorError::TaskIdInvalidFormat { .. } => StatusCode::BAD_REQUEST,
            SupervisorError::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> String {
        self.to_string()
    }
}

impl toy_core::error::Error for SupervisorError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SupervisorError::Error {
            inner: msg.to_string(),
        }
    }
}

impl toy_api_http_common::axum::response::IntoResponse for SupervisorError {
    fn into_response(self) -> Response {
        let e = ErrorMessage::new(self.status_code().as_u16(), self.error_message());
        let code = StatusCode::from_u16(e.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let r = reply::into_response(&e, Some(Format::Json), None);
        (code, r).into_response()
    }
}

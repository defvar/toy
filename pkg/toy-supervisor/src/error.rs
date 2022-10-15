use std::fmt::Display;
use thiserror::Error;
use toy_api::error::ErrorMessage;
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::Response;
use toy_core::error::ConfigError;

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error(transparent)]
    ApiHttpCommonError {
        #[from]
        source: toy_api_http_common::Error,
    },

    #[error(transparent)]
    ParseGraphConfigFailed {
        #[from]
        source: ConfigError,
    },

    #[error("{:?}", inner)]
    Error { inner: String },
}

impl SupervisorError {
    pub fn error<T>(msg: T) -> SupervisorError
    where
        T: Display,
    {
        SupervisorError::Error {
            inner: msg.to_string(),
        }
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
        let e = ErrorMessage::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "");
        let json = toy_pack_json::pack_to_string(&e);
        match json {
            Ok(v) => (StatusCode::INTERNAL_SERVER_ERROR, v).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()).into_response(),
        }
    }
}

use std::fmt::Display;
use thiserror::Error;
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

impl toy_api_http_common::warp::reject::Reject for SupervisorError {}

use thiserror::Error;

use toy_api::error::ErrorMessage;
#[cfg(feature = "http")]
use toy_api_http_common::Error;
#[cfg(feature = "http")]
use toy_h::error::HError;
#[cfg(feature = "http")]
use toy_h::InvalidUri;
#[cfg(feature = "http")]
use toy_h::StatusCode;
#[cfg(feature = "http")]
use toy_pack_urlencoded::QueryParseError;

#[derive(Debug, Error)]
pub enum ApiClientError {
    #[error("authentication failed. {}", inner)]
    AuthenticationFailed { inner: String },

    #[error(transparent)]
    DeserializeJsonValue {
        #[from]
        source: toy_pack_json::DecodeError,
    },

    #[error(transparent)]
    SerializeJsonValue {
        #[from]
        source: toy_pack_json::EncodeError,
    },

    #[error(transparent)]
    DeserializeMessagePackValue {
        #[from]
        source: toy_pack_mp::DecodeError,
    },

    #[error(transparent)]
    SerializeMessagePackValue {
        #[from]
        source: toy_pack_mp::EncodeError,
    },

    #[cfg(feature = "http")]
    #[error(transparent)]
    InvalidUri {
        #[from]
        source: InvalidUri,
    },

    #[cfg(feature = "http")]
    #[error(transparent)]
    HError {
        #[from]
        source: HError,
    },

    #[cfg(feature = "http")]
    #[error(transparent)]
    QueryParse {
        #[from]
        source: QueryParseError,
    },

    #[cfg(feature = "http")]
    #[error(transparent)]
    ApiError {
        #[from]
        source: toy_api_http_common::Error,
    },
}

impl ApiClientError {
    #[cfg(feature = "http")]
    pub fn query_parse(e: QueryParseError) -> ApiClientError {
        ApiClientError::QueryParse { source: e }
    }

    #[cfg(feature = "http")]
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiClientError::ApiError { source } => source.status_code(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            ApiClientError::ApiError { source } => source.error_message(),
            _ => self.to_string(),
        }
    }
}

impl From<ErrorMessage> for ApiClientError {
    fn from(e: ErrorMessage) -> Self {
        ApiClientError::ApiError {
            source: Error::ApiError { inner: e },
        }
    }
}

use thiserror::Error;

use toy_api::error::ErrorMessage;
#[cfg(feature = "http")]
use toy_h::error::HError;
#[cfg(feature = "http")]
use toy_h::InvalidUri;
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

    #[error("code: {}, message: {}", inner.code(), inner.message())]
    ApiError { inner: ErrorMessage },
}

impl ApiClientError {
    #[cfg(feature = "http")]
    pub fn query_parse(e: QueryParseError) -> ApiClientError {
        ApiClientError::QueryParse { source: e }
    }
}

impl From<ErrorMessage> for ApiClientError {
    fn from(e: ErrorMessage) -> Self {
        ApiClientError::ApiError { inner: e }
    }
}

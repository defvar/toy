use crate::models::ErrorInfo;
use std::fmt::Display;
use thiserror::Error as ThisError;
use toy_h::InvalidUri;

#[derive(Debug, ThisError)]
pub enum InfluxDBError {
    #[error("error: {:?}", source)]
    DeserializeError {
        #[from]
        source: toy_pack_json::DecodeError,
    },

    #[error("error: {:?}", source)]
    SerializeError {
        #[from]
        source: toy_pack_json::EncodeError,
    },

    #[error("error: {:?}", source)]
    HError {
        #[from]
        source: toy_h::error::HError,
    },

    #[error(transparent)]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error(transparent)]
    Utf8Error {
        #[from]
        source: std::str::Utf8Error,
    },

    #[error("invalid uri: {:?}", source)]
    InvalidUri {
        #[from]
        source: InvalidUri,
    },

    #[error(
        "invalid field value: field: {}, expected:{}, value:{}",
        field,
        expected_type,
        value
    )]
    InvalidFieldValue {
        field: String,
        expected_type: String,
        value: String,
    },

    #[error("error: {:?}", info)]
    InfluxDBApiError { info: ErrorInfo },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl InfluxDBError {
    pub fn error<T>(msg: T) -> InfluxDBError
    where
        T: Display,
    {
        InfluxDBError::Error {
            inner: msg.to_string(),
        }
    }

    pub fn invalid_field_value(
        field: impl Into<String>,
        expected_type: impl Into<String>,
        value: impl Into<String>,
    ) -> InfluxDBError {
        InfluxDBError::InvalidFieldValue {
            field: field.into(),
            expected_type: expected_type.into(),
            value: value.into(),
        }
    }

    pub fn api_error(info: ErrorInfo) -> InfluxDBError {
        InfluxDBError::InfluxDBApiError { info }
    }
}

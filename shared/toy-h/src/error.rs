use http::uri::InvalidUri;
use std::fmt::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum HError {
    #[error("invalid uri: {:?}", source)]
    InvalidUri {
        #[from]
        source: InvalidUri,
    },

    #[error("error: {:?}", inner)]
    Error { inner: String },
}

impl HError {
    pub fn error<T>(msg: T) -> HError
    where
        T: Display,
    {
        HError::Error {
            inner: msg.to_string(),
        }
    }
}

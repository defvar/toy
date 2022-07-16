use std::backtrace::Backtrace;
use std::net::AddrParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error: invalid log path")]
    InvalidLogPath,

    #[error("error: {:?}", source)]
    IOError {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("error: {:?}", source)]
    AddrParseError {
        #[from]
        source: AddrParseError,
        backtrace: Backtrace,
    },
}

impl Error {
    pub fn invalid_log_path() -> Error {
        Error::InvalidLogPath
    }
}

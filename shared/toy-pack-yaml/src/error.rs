use std::fmt::Display;

use toy_pack::deser::Error;

#[derive(Debug, Fail)]
pub enum YamlError {
    #[fail(display = "yaml error: {:?}", inner)]
    ScanError {
        inner: yaml_rust::ScanError
    },

    #[fail(display = "yaml error: {:?}", inner)]
    Error {
        inner: String,
    },
}

impl YamlError {
    pub fn error<T>(msg: T) -> YamlError where T: Display {
        YamlError::Error {
            inner: msg.to_string(),
        }
    }
}

impl From<yaml_rust::ScanError> for YamlError {
    fn from(e: yaml_rust::ScanError) -> Self {
        YamlError::ScanError {
            inner: e,
        }
    }
}

impl Error for YamlError {
    fn custom<T>(msg: T) -> Self where T: Display {
        YamlError::error(msg)
    }
}

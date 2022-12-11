use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorInfo {
    code: String,
    message: String,
}

impl ErrorInfo {
    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.code, self.message))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Annotation {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "datatype")]
    DataType,
    #[serde(rename = "default")]
    Default,
}

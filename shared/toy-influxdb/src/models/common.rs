use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Annotation {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "datatype")]
    DataType,
    #[serde(rename = "default")]
    Default,
}

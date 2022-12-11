use crate::models::common::Annotation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryParam {
    dialect: Dialect,
    query: String,
    #[serde(rename = "type")]
    tp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dialect {
    annotations: Vec<Annotation>,
    #[serde(rename = "commentPrefix")]
    comment_prefix: String,
    #[serde(rename = "dateTimeFormat")]
    date_time_format: DateTimeFormat,
    delimiter: String,
    header: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DateTimeFormat {
    RFC3339,
    RFC3339Nano,
}

impl QueryParam {
    pub fn with(query: String) -> QueryParam {
        QueryParam {
            dialect: Dialect::default(),
            query,
            tp: "flux".to_string(),
        }
    }
}

impl Default for Dialect {
    fn default() -> Self {
        Self {
            annotations: vec![Annotation::Group, Annotation::DataType, Annotation::Default],
            comment_prefix: "#".to_string(),
            date_time_format: DateTimeFormat::RFC3339Nano,
            delimiter: ",".to_string(),
            header: true,
        }
    }
}

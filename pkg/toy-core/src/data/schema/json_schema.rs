use crate::data::Map;
use toy_pack::Pack;

#[derive(Clone, Copy, Debug, PartialEq, Pack)]
pub enum SchemaTypes {
    #[toy(rename = "array")]
    Array,
    #[toy(rename = "boolean")]
    Boolean,
    #[toy(rename = "integer")]
    Integer,
    #[toy(rename = "null")]
    Null,
    #[toy(rename = "number")]
    Number,
    #[toy(rename = "object")]
    Object,
    #[toy(rename = "string")]
    String,
}

#[derive(Clone, Debug, Pack)]
#[toy(ignore_ser_if_none)]
pub struct JsonSchema {
    #[toy(rename = "$id")]
    pub(crate) id: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) properties: Option<Map<String, JsonSchema>>,
    pub(crate) items: Option<Vec<JsonSchema>>,
    pub(crate) required: Option<Vec<String>>,
    #[toy(rename = "type")]
    pub(crate) tp: Option<SchemaTypes>,

    #[toy(rename = "oneOf")]
    pub(crate) one_of: Option<Vec<JsonSchema>>,
    #[toy(rename = "const")]
    pub(crate) const_: Option<String>,

    #[toy(rename = "additionalProperties")]
    pub(crate) additional_properties: Option<Vec<JsonSchema>>,

    #[toy(ignore)]
    pub(crate) is_optional: bool,
}

impl JsonSchema {
    pub fn new() -> JsonSchema {
        JsonSchema {
            id: None,
            title: None,
            properties: None,
            items: None,
            required: None,
            tp: None,
            one_of: None,
            const_: None,
            additional_properties: None,
            is_optional: false,
        }
    }

    pub fn from_types(tp: SchemaTypes) -> JsonSchema {
        JsonSchema {
            tp: Some(tp),
            ..JsonSchema::new()
        }
    }

    pub fn tp(&self) -> Option<SchemaTypes> {
        self.tp.clone()
    }

    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    pub fn into_optional(self) -> JsonSchema {
        JsonSchema {
            is_optional: true,
            ..self
        }
    }

    pub fn into_required(self, required: Vec<String>) -> JsonSchema {
        let req = if required.len() > 0 {
            Some(required)
        } else {
            None
        };
        JsonSchema {
            required: req,
            ..self
        }
    }
}

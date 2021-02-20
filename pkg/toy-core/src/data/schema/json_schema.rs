use crate::data::Map;
use toy_pack::deser::{Deserializable, Deserializer, Error, Visitor};
use toy_pack::ser::{Serializable, Serializer};
use toy_pack::{Pack, Unpack};

/// JsonSchema Object Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pack, Unpack)]
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

/// JsonSchema Structure.
#[derive(Clone, Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
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
    pub(crate) additional_properties: Option<Box<JsonSchema>>,

    pub(crate) minimum: Option<RangeValue>,
    pub(crate) maximum: Option<RangeValue>,

    #[toy(rename = "minLength")]
    pub(crate) min_length: Option<u64>,
    #[toy(rename = "maxLength")]
    pub(crate) max_length: Option<u64>,

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
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
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

/// A range which can be used to 'minimum' or 'maximum'.
#[derive(Debug, Clone, PartialEq)]
pub enum RangeValue {
    U64(u64),
    I64(i64),
    F64(f64),
}

impl Serializable for RangeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RangeValue::U64(v) => serializer.serialize_u64(*v),
            RangeValue::I64(v) => serializer.serialize_i64(*v),
            RangeValue::F64(v) => serializer.serialize_f64(*v),
        }
    }
}

struct RangeValueVisitor;

impl<'a> Visitor<'a> for RangeValueVisitor {
    type Value = RangeValue;
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::U64(v as u64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::U64(v as u64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::U64(v as u64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::U64(v as u64))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::I64(v as i64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::I64(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::I64(v as i64))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::I64(v as i64))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::F64(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RangeValue::F64(v as f64))
    }
}

impl<'toy> Deserializable<'toy> for RangeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_any(RangeValueVisitor)
    }
}

use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt::{self, Debug};

mod de;
mod ser;

#[derive(Clone, PartialEq)]
pub enum JValue {
    Null,
    Bool(bool),
    String(String),
    Integer(i64),
    Number(f64),
    Array(Vec<JValue>),
    Object(IndexMap<String, JValue>),
}

impl JValue {
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            JValue::Null => Some(()),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            JValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<JValue>> {
        match *self {
            JValue::Array(ref vec) => Some(vec),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&IndexMap<String, JValue>> {
        match *self {
            JValue::Object(ref map) => Some(map),
            _ => None,
        }
    }
}

impl From<HashMap<String, String>> for JValue {
    fn from(v: HashMap<String, String>) -> Self {
        let mut map = IndexMap::with_capacity(v.len());
        for (k, v) in v {
            map.insert(k, JValue::String(v));
        }
        JValue::Object(map)
    }
}

impl Debug for JValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JValue::Null => formatter.debug_tuple("Null").finish(),
            JValue::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            JValue::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            JValue::Integer(ref v) => Debug::fmt(v, formatter),
            JValue::Number(ref v) => Debug::fmt(v, formatter),
            JValue::Array(ref v) => {
                formatter.write_str("Array(")?;
                Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
            JValue::Object(ref v) => {
                formatter.write_str("Object(")?;
                Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
        }
    }
}

impl Default for JValue {
    fn default() -> JValue {
        JValue::Null
    }
}

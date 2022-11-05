//! Function to narrow down the structure to only the necessary items.

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashSet;
use std::fmt::Formatter;
use toy_core::data::{Map, Value};

/// Fields infomation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Fields {
    fields: HashSet<String>,
}

impl Fields {
    pub fn from_vec(fields: Vec<String>) -> Fields {
        Fields {
            fields: HashSet::from_iter(fields),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn apply(&self, v: &Value) -> Result<Value, String> {
        let mut r = Value::Map(Map::with_capacity(self.fields.len()));
        for f in &self.fields {
            match v.path(f) {
                Some(fv) => {
                    let _ = r.insert_by_path(f, fv.clone());
                }
                None => return Err(f.to_string()), //missing field
            }
        }
        Ok(r)
    }
}

impl Default for Fields {
    fn default() -> Self {
        Fields {
            fields: HashSet::new(),
        }
    }
}

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FieldsVisitor)
    }
}

struct FieldsVisitor;

impl<'de> Visitor<'de> for FieldsVisitor {
    type Value = Fields;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("fields")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let fields: HashSet<_> = v.split(",").map(|x| x.to_string()).collect();
        Ok(Fields { fields })
    }
}

#[cfg(test)]
mod tests {
    use crate::selection::fields::Fields;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
    struct Test {
        f: Fields,
    }

    #[test]
    fn deser_fields() {
        let t: Test = toy_pack_json::unpack("{ \"f\": \"abc,def\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.f,
            Fields::from_vec(vec!["abc".to_string(), "def".to_string()])
        );
    }
}

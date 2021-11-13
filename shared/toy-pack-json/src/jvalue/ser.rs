use crate::jvalue::JValue;
use serde::{Serialize, Serializer};

impl Serialize for JValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            JValue::Null => serializer.serialize_unit(),
            JValue::Bool(b) => serializer.serialize_bool(b),
            JValue::Number(ref n) => n.serialize(serializer),
            JValue::Integer(ref n) => n.serialize(serializer),
            JValue::String(ref s) => serializer.serialize_str(s),
            JValue::Array(ref v) => v.serialize(serializer),
            JValue::Object(ref m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

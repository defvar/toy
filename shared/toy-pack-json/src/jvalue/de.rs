use crate::jvalue::JValue;
use indexmap::IndexMap;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::{self};

impl<'de> Deserialize<'de> for JValue {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = JValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<JValue, E> {
                Ok(JValue::Bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<JValue, E> {
                Ok(JValue::Integer(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<JValue, E> {
                use toy_pack::FromPrimitive;
                Ok(i64::from_u64(value).map_or(JValue::Null, JValue::Integer))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<JValue, E> {
                Ok(JValue::Number(value))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<JValue, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<JValue, E> {
                Ok(JValue::String(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<JValue, E> {
                Ok(JValue::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<JValue, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<JValue, E> {
                Ok(JValue::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<JValue, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }
                Ok(JValue::Array(vec))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<JValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut values: IndexMap<String, JValue> = IndexMap::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }
                Ok(JValue::Object(values))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

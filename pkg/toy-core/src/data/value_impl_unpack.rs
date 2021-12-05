use crate::data::error::DeserializeError;
use crate::data::Value;
use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use std::fmt::Formatter;
use toy_map::Map;

/// Deserialize from `Value`
///
/// # Exapmle
///
/// ```edition2018
/// use serde::Deserialize;
/// use toy_core::prelude::*;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct User {
///     id: u32,
///     name: String
/// }
///
/// fn main(){
///
///     // struct is defined by map.
///     let mut map = Map::new();
///     map.insert("id".to_string(), Value::from(123u32));
///     map.insert("name".to_string(), Value::from("aiueo".to_string()));
///
///     let src = Value::from(map);
///
///     let dest = data::unpack::<User>(&src).unwrap();
///
///     assert_eq!(dest, User { id: 123u32, name: "aiueo".to_string()})
/// }
/// ```
#[inline]
pub fn unpack<'toy, T>(v: &'toy Value) -> Result<T, DeserializeError>
where
    T: Deserialize<'toy>,
{
    T::deserialize(&mut ValueDeserializer { value: v })
}

macro_rules! from_value {
    ($t: ident, $variant: ident) => {
        fn $variant<E>(self, v: $t) -> Result<Self::Value, E> {
            Ok(Value::from(v))
        }
    };
}

pub struct ValueDeserializer<'a> {
    value: &'a Value,
}

impl<'toy: 'a, 'a> Deserialize<'toy> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct ValueVisitor;

        impl<'a> Visitor<'a> for ValueVisitor {
            type Value = Value;

            from_value!(bool, visit_bool);
            from_value!(u8, visit_u8);
            from_value!(u16, visit_u16);
            from_value!(u32, visit_u32);
            from_value!(u64, visit_u64);
            from_value!(i8, visit_i8);
            from_value!(i16, visit_i16);
            from_value!(i32, visit_i32);
            from_value!(i64, visit_i64);
            from_value!(f32, visit_f32);
            from_value!(f64, visit_f64);

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::from(v))
            }

            fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::from(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::from(v))
            }

            fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_bytes(v)
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_bytes(&v)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'a>,
            {
                Value::deserialize(deserializer).map(|x| x)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::None)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'a>,
            {
                let size = seq.size_hint().unwrap_or(256);
                let mut vec: Vec<Value> = Vec::with_capacity(size);
                while let Some(item) = seq.next_element::<Value>()? {
                    vec.push(item);
                }
                Ok(Value::from(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'a>,
            {
                let size = map.size_hint().unwrap_or(256);
                let mut values: Map<String, Value> = Map::with_capacity(size);
                while let Some(key) = map.next_key::<String>()? {
                    let v = map.next_value::<Value>()?;
                    values.insert(key, v);
                }
                Ok(Value::from(values))
            }

            fn visit_enum<A>(self, _data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'a>,
            {
                Err(Error::custom("enum not support"))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

macro_rules! de_value_number {
    ($t: ident, $func: ident, $visit: ident, $variant: ident, $expected: literal) => {
        fn $func<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'toy>,
        {
            match self.value.parse_integer::<$t>() {
                Some(v) => visitor.$visit(v),
                None => Err(DeserializeError::invalid_type($expected, self.value)),
            }
        }
    };
}

impl<'toy, 'a> Deserializer<'toy> for &'a mut ValueDeserializer<'toy> {
    type Error = DeserializeError;

    de_value_number!(u8, deserialize_u8, visit_u8, Integer, "u8");
    de_value_number!(u16, deserialize_u16, visit_u16, Integer, "u16");
    de_value_number!(u32, deserialize_u32, visit_u32, Integer, "u32");
    de_value_number!(u64, deserialize_u64, visit_u64, Integer, "u64");
    de_value_number!(i8, deserialize_i8, visit_i8, Integer, "i8");
    de_value_number!(i16, deserialize_i16, visit_i16, Integer, "i16");
    de_value_number!(i32, deserialize_i32, visit_i32, Integer, "i32");
    de_value_number!(i64, deserialize_i64, visit_i64, Integer, "i64");

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::Integer(v) => visitor.visit_i64(*v),
            Value::Number(v) => visitor.visit_f64(*v),
            Value::String(ref v) => visitor.visit_str(v),
            Value::Bytes(v) => visitor.visit_bytes(v.as_slice()),
            Value::None => self.deserialize_option(visitor),
            Value::Seq(_) => self.deserialize_seq(visitor),
            Value::Map(_) => self.deserialize_map(visitor),
            Value::TimeStamp(_) => self.deserialize_map(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::Bool(v) => visitor.visit_bool(*v),
            _ => Err(DeserializeError::invalid_type("bool", self.value)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_f32(
            self.value
                .parse_f32()
                .ok_or(DeserializeError::invalid_type("f32", self.value))?,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_f64(
            self.value
                .parse_f64()
                .ok_or(DeserializeError::invalid_type("f64", self.value))?,
        )
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::String(v) => visitor.visit_str(v),
            _ => Err(DeserializeError::invalid_type("char", self.value)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::String(v) => visitor.visit_str(v.as_str()),
            _ => Err(DeserializeError::invalid_type("str", self.value)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::String(v) => visitor.visit_str(v),
            _ => Err(DeserializeError::invalid_type("str", self.value)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::Bytes(v) => visitor.visit_bytes(v.as_slice()),
            _ => Err(DeserializeError::invalid_type("bytes", self.value)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(DeserializeError::invalid_type("unit", self.value))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::Seq(_) => {
                let seq = DeserializeSeq::new(self);
                visitor.visit_seq(seq)
            }
            _ => Err(DeserializeError::invalid_type("seq", self.value)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::Map(_) => visitor.visit_map(DeserializeMap::new(self)),
            Value::TimeStamp(_) => {
                // TODO ...
                visitor.visit_map(DeserializeMap::new(self))
            }
            _ => Err(DeserializeError::invalid_type("map", self.value)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            Value::Seq(_) => self.deserialize_seq(visitor),
            Value::Map(_) => self.deserialize_map(visitor),
            Value::TimeStamp(_) => self.deserialize_map(visitor),
            _ => Err(DeserializeError::error(
                "deserialize struct, must be a map, array or Duration type.",
            )),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &[&str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.value {
            v @ Value::Map(_) => {
                let len = v.as_map().unwrap().len();
                if len == 1 {
                    visitor.visit_enum(DeserializeVariant::new(self))
                } else {
                    Err(DeserializeError::error(format!(
                        "Oops, map length:{}. When deserializing an enum from a 'map', length must be 1.", len
                    )))
                }
            }
            Value::String(_) => visitor.visit_enum(DeserializeVariant::new(self)),
            _ => Err(DeserializeError::error(
                "deserialize enum, must be a map type.",
            )),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_unit()
    }
}

struct DeserializeSeq<'r, 'toy: 'r> {
    de: &'r mut ValueDeserializer<'toy>,
    idx: usize,
    remaining: usize,
}

impl<'r, 'toy> DeserializeSeq<'r, 'toy> {
    pub fn new(de: &'r mut ValueDeserializer<'toy>) -> Self {
        let remaining = de.value.as_vec().unwrap().len();
        Self {
            de,
            idx: 0,
            remaining,
        }
    }
}

impl<'toy, 'r> SeqAccess<'toy> for DeserializeSeq<'r, 'toy> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            let v = self.de.value.as_vec().unwrap().get(self.idx).unwrap();
            self.idx += 1;
            seed.deserialize(&mut ValueDeserializer { value: v })
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct DeserializeMap<'r, 'a: 'r> {
    de: &'r mut ValueDeserializer<'a>,
    value: Option<&'a Value>,
    keys: Vec<&'a String>,
    index: usize,
    remaining: usize,
}

impl<'r, 'a> DeserializeMap<'r, 'a> {
    pub fn new(de: &'r mut ValueDeserializer<'a>) -> Self {
        let keys = de.value.as_map().unwrap().keys().collect::<Vec<_>>();
        let len = keys.len();
        Self {
            de,
            value: None,
            keys,
            index: 0,
            remaining: len,
        }
    }
}

impl<'toy, 'a> MapAccess<'toy> for DeserializeMap<'a, 'toy> {
    type Error = DeserializeError;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            let key = self.keys.get(self.index).unwrap();
            self.index += 1;
            match self.de.value.as_map().unwrap().get(key.as_str()) {
                Some(v) => {
                    self.value = Some(v);
                    seed.deserialize(&mut IdDeserializer {
                        v: Value::from(key.as_str()),
                    })
                    .map(Some)
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        seed.deserialize(&mut ValueDeserializer { value })
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

struct DeserializeVariant<'r, 'a: 'r> {
    de: &'r mut ValueDeserializer<'a>,
    value: Option<&'a Value>,
}

impl<'r, 'a> DeserializeVariant<'r, 'a> {
    pub fn new(de: &'r mut ValueDeserializer<'a>) -> Self {
        Self { de, value: None }
    }
}

impl<'toy, 'a> VariantAccess<'toy> for DeserializeVariant<'a, 'toy> {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        seed.deserialize(&mut ValueDeserializer { value })
    }

    fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        serde::de::Deserializer::deserialize_seq(&mut ValueDeserializer { value }, visitor)
    }

    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        serde::de::Deserializer::deserialize_map(&mut ValueDeserializer { value }, visitor)
    }
}

impl<'toy, 'a> EnumAccess<'toy> for DeserializeVariant<'a, 'toy> {
    type Error = DeserializeError;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'toy>,
    {
        let v = match self.de.value.as_map() {
            Some(map) => {
                let (k, v) = map.iter().next().unwrap();
                self.value = Some(v);
                seed.deserialize(&mut IdDeserializer {
                    v: Value::from(k.as_str()),
                })?
            }
            None => seed.deserialize(&mut IdDeserializer {
                v: Value::from(self.de.value.as_str().unwrap()),
            })?,
        };
        Ok((v, self))
    }
}

pub struct IdDeserializer {
    v: Value,
}

impl<'a, 'b> Deserializer<'b> for &'a mut IdDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        unimplemented!("IdDeserializer may only be used for identifiers")
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        match &self.v {
            Value::String(v) => visitor.visit_str(v.as_str()),
            _ => Err(DeserializeError::invalid_type("str", &self.v)),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'b>,
    {
        self.deserialize_any(visitor)
    }
}

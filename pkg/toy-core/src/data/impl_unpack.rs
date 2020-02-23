use crate::data::error::DeserializeError;
use crate::data::Value;
use std::collections::HashMap;
use toy_pack::deser::{
    Deserializable, DeserializeMapOps, DeserializeSeqOps, DeserializeVariantOps, Deserializer,
    Error, Visitor,
};

/// Deserialize from `Value`
///
/// # Exapmle
///
/// ```edition2018
/// use std::collections::HashMap;
/// use toy_pack_derive::*;
/// use toy_core::data::{self, Value};
///
/// #[derive(UnPack)]
/// struct User {
///   id: u32,
///   name: String
/// }
///
/// fn main(){
///
///   // struct is defined by map.
///   let mut map = HashMap::new();
///   map.insert("id".to_string(), Value::from(123u32));
///   map.insert("name".to_string(), Value::from("aiueo".to_string()));
///
///   let v = Value::from(map);
///   data::unpack::<User>(v).unwrap();
/// }
/// ```
#[inline]
pub fn unpack<'toy, T>(v: Value) -> Result<T::Value, DeserializeError>
where
    T: Deserializable<'toy>,
{
    T::deserialize(v)
}

macro_rules! from_value {
    ($t: ident, $variant: ident) => {
       fn $variant<E>(self, v: $t) -> Result<Self::Value, E> {
            Ok(Value::from(v))
        }
    };
}

impl<'toy: 'a, 'a> Deserializable<'toy> for Value {
    type Value = Value;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
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

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: DeserializeSeqOps<'a>,
            {
                let size = seq.size_hint().unwrap_or(256);
                let mut vec: Vec<Value> = Vec::with_capacity(size);
                while let Some(item) = seq.next::<Value>()? {
                    vec.push(item);
                }
                Ok(Value::from(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: DeserializeMapOps<'a>,
            {
                let size = map.size_hint().unwrap_or(256);
                let mut values: HashMap<String, Value> = HashMap::with_capacity(size);
                while let Some(key) = map.next_key::<String>()? {
                    let v = map.next_value::<Value>()?;
                    values.insert(key, v);
                }
                Ok(Value::from(values))
            }

            fn visit_enum<A>(self, _data: A) -> Result<Self::Value, A::Error>
            where
                A: DeserializeVariantOps<'a>,
            {
                Err(Error::custom("enum not support"))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'a>,
            {
                Value::deserialize(deserializer).map(|x| Value::Some(Box::new(x)))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Unit)
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

macro_rules! de_value {
    ($t: ident, $func: ident, $variant: ident, $expected: literal) => {
       fn $func(self) -> Result<$t, Self::Error> {
           match self {
               Value::$variant(v) => Ok(v),
               _ => Err(DeserializeError::invalid_type($expected, self)),
           }
        }
    };
}

impl<'toy> Deserializer<'toy> for Value {
    type Error = DeserializeError;

    de_value!(bool, deserialize_bool, Bool, "bool");
    de_value!(u8, deserialize_u8, U8, "u8");
    de_value!(u16, deserialize_u16, U16, "u16");
    de_value!(u32, deserialize_u32, U32, "u32");
    de_value!(u64, deserialize_u64, U64, "u64");
    de_value!(i8, deserialize_i8, I8, "i8");
    de_value!(i16, deserialize_i16, I16, "i16");
    de_value!(i32, deserialize_i32, I32, "i32");
    de_value!(i64, deserialize_i64, I64, "i64");
    de_value!(f32, deserialize_f32, F32, "f32");
    de_value!(f64, deserialize_f64, F64, "f64");

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Char(v) => visitor.visit_char(v),
            _ => Err(DeserializeError::invalid_type("char", self)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::String(v) => visitor.visit_str(v.as_str()),
            _ => Err(DeserializeError::invalid_type("str", self)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::String(v) => visitor.visit_string(v),
            _ => Err(DeserializeError::invalid_type("str", self)),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Seq(vec) => {
                let seq = DeserializeSeq::new(vec);
                visitor.visit_seq(seq)
            }
            _ => Err(DeserializeError::invalid_type("seq", self)),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Map(map) => {
                let len = map.keys().len();
                let iter = map.into_iter();
                visitor.visit_map(DeserializeMap::new(iter, len))
            }
            _ => Err(DeserializeError::invalid_type("map", self)),
        }
    }

    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Seq(_) => self.deserialize_seq(visitor),
            Value::Map(_) => self.deserialize_map(visitor),
            _ => Err(DeserializeError::error(
                "deserialize struct, must be a map type or array type.",
            )),
        }
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Map(map) => {
                let len = map.len();
                if len == 1 {
                    let len = map.keys().len();
                    let iter = map.into_iter();
                    visitor.visit_enum(DeserializeMap::new(iter, len))
                } else {
                    Err(DeserializeError::error(format!(
                        "Oops, map length:{}. When deserializing an enum from a 'map', length must be 1.", len
                    )))
                }
            }
            _ => Err(DeserializeError::error(
                "deserialize enum, must be a map type.",
            )),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Some(v) => visitor.visit_some(*v),
            Value::None => visitor.visit_none(),
            _ => Err(DeserializeError::invalid_type("option", self)),
        }
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self {
            Value::Bool(v) => visitor.visit_bool(v),
            Value::U8(v) => visitor.visit_u8(v),
            Value::U16(v) => visitor.visit_u16(v),
            Value::U32(v) => visitor.visit_u32(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::I8(v) => visitor.visit_i8(v),
            Value::I16(v) => visitor.visit_i16(v),
            Value::I32(v) => visitor.visit_i32(v),
            Value::I64(v) => visitor.visit_i64(v),
            Value::F32(v) => visitor.visit_f32(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::Char(v) => visitor.visit_char(v),
            Value::String(v) => visitor.visit_string(v),
            Value::Bytes(_) => unimplemented!(),
            Value::None | Value::Some(_) => self.deserialize_option(visitor),
            Value::Seq(_) => self.deserialize_seq(visitor),
            Value::Map(_) => self.deserialize_map(visitor),
            Value::Unit => visitor.visit_unit(),
        }
    }

    fn discard<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        drop(self);
        visitor.visit_unit()
    }
}

struct DeserializeSeq {
    vec: Vec<Value>,
    idx: usize,
    remaining: usize,
}

impl DeserializeSeq {
    pub fn new(value: Vec<Value>) -> Self {
        let remaining = value.len();
        Self {
            vec: value,
            idx: 0,
            remaining,
        }
    }
}

impl<'toy> DeserializeSeqOps<'toy> for DeserializeSeq {
    type Error = DeserializeError;

    fn next<T>(&mut self) -> Result<Option<T::Value>, Self::Error>
    where
        T: Deserializable<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            let v = self.vec.get(self.idx).unwrap().clone();
            self.idx += 1;
            T::deserialize(v).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct DeserializeMap<I> {
    iter: I,
    value: Option<Value>,
    remaining: usize,
}

impl<I> DeserializeMap<I> {
    pub fn new(iter: I, size: usize) -> Self {
        Self {
            iter,
            value: None,
            remaining: size,
        }
    }
}

impl<'toy, I> DeserializeMapOps<'toy> for DeserializeMap<I>
where
    I: Iterator<Item = (String, Value)>,
{
    type Error = DeserializeError;

    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                visitor.visit_string(k).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_key<T>(&mut self) -> Result<Option<T::Value>, Self::Error>
    where
        T: Deserializable<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                T::deserialize(Value::from(k)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value<T>(&mut self) -> Result<T::Value, Self::Error>
    where
        T: Deserializable<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        T::deserialize(value)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

impl<'toy, I> DeserializeVariantOps<'toy> for DeserializeMap<I>
where
    I: Iterator<Item = (String, Value)>,
{
    type Error = DeserializeError;

    fn variant_identifier<V>(mut self, visitor: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                visitor.visit_string(k).map(|x| (x, self))
            }
            None => visitor.visit_none().map(|x| (x, self)),
        }
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant<T>(mut self) -> Result<T::Value, Self::Error>
    where
        T: Deserializable<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        T::deserialize(value)
    }

    fn tuple_variant<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        toy_pack::deser::Deserializer::deserialize_seq(value, visitor)
    }
}

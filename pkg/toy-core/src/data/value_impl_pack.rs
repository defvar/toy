use crate::data::error::SerializeError;
use crate::data::{Map, Value};
use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

/// Serialize to `Value`
///
/// # Example
///
/// ```edition2018
/// use serde::Serialize;
/// use toy_core::prelude::*;
///
/// #[derive(Debug, PartialEq, Serialize)]
/// struct User {
///     id: u32,
///     name: String
/// }
///
/// fn main(){
///     let src = User {
///         id: 123u32,
///         name: "aiueo".to_string(),
///     };
///
///     let dest = data::pack(src).unwrap();
///
///     assert_eq!(dest, map_value! {
///         "id" => 123u32,
///         "name" => "aiueo"
///     });
/// }
/// ```
#[inline]
pub fn pack<T>(v: T) -> Result<Value, SerializeError>
where
    T: Serialize,
{
    let mut buf = Value::default();
    v.serialize(&mut buf)?;
    Ok(buf)
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::U8(v) => serializer.serialize_u8(*v),
            Value::U16(v) => serializer.serialize_u16(*v),
            Value::U32(v) => serializer.serialize_u32(*v),
            Value::U64(v) => serializer.serialize_u64(*v),
            Value::I8(v) => serializer.serialize_i8(*v),
            Value::I16(v) => serializer.serialize_i16(*v),
            Value::I32(v) => serializer.serialize_i32(*v),
            Value::I64(v) => serializer.serialize_i64(*v),
            Value::F32(v) => serializer.serialize_f32(*v),
            Value::F64(v) => serializer.serialize_f64(*v),
            Value::String(v) => serializer.serialize_str(v),
            Value::Bytes(v) => serializer.serialize_bytes(v.as_slice()),
            Value::None => serializer.serialize_none(),
            Value::Seq(v) => serializer.collect_seq(v),
            Value::Map(v) => serializer.collect_map(v),
            Value::TimeStamp(v) => v.serialize(serializer),
        }
    }
}

impl<'a> Serializer for &'a mut Value {
    type Ok = ();
    type Error = SerializeError;
    type SerializeSeq = SerializeCompound<'a>;
    type SerializeTuple = SerializeCompound<'a>;
    type SerializeTupleStruct = SerializeCompound<'a>;
    type SerializeTupleVariant = SerializeTupleVariantImpl<'a>;
    type SerializeMap = SerializeCompound<'a>;
    type SerializeStruct = SerializeCompound<'a>;
    type SerializeStructVariant = SerializeStructVariantImpl<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        *self = Value::from(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        *self = Value::None;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut buf = Value::default();
        v.serialize(&mut buf)?;
        *self = buf;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut buf = Value::default();
        value.serialize(&mut buf)?;
        *self = Value::from(buf);
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut map = Map::with_capacity(1);
        let mut buf = Value::default();
        value.serialize(&mut buf)?;
        map.insert(variant.to_string(), buf);
        *self = Value::from(map);
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        *self = default_empty_seq(len);
        Ok(SerializeCompound::new(self, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariantImpl::new(self, variant, len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        *self = default_empty_map(len);
        Ok(SerializeCompound::new(self, len))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeCompound::new(self, Some(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariantImpl::new(self, variant, len))
    }
}

pub struct SerializeCompound<'a> {
    ser: &'a mut Value,

    // key buffer for map ops
    key: Option<Value>,
    len: Option<usize>,
}

pub struct SerializeTupleVariantImpl<'a> {
    ser: &'a mut Value,
    name: &'static str,
    len: usize,
    seq: Value,
}

pub struct SerializeStructVariantImpl<'a> {
    ser: &'a mut Value,
    name: &'static str,
    len: usize,
    map: Value,
}

impl<'a> SerializeCompound<'a> {
    pub fn new(ser: &'a mut Value, len: Option<usize>) -> Self {
        Self {
            ser,
            key: None,
            len,
        }
    }
}

impl<'a> SerializeTupleVariantImpl<'a> {
    pub fn new(ser: &'a mut Value, name: &'static str, len: usize) -> Self {
        Self {
            ser,
            name,
            seq: Value::default(),
            len,
        }
    }
}

impl<'a> SerializeStructVariantImpl<'a> {
    pub fn new(ser: &'a mut Value, name: &'static str, len: usize) -> Self {
        Self {
            ser,
            name,
            map: Value::default(),
            len,
        }
    }
}

impl<'a> SerializeSeq for SerializeCompound<'a> {
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut buf = Value::default();
        value.serialize(&mut buf)?;
        match self.ser {
            Value::Seq(ref mut seq) => {
                seq.push(buf);
            }
            _ => {
                *self.ser = default_seq(buf, self.len);
            }
        }
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeMap for SerializeCompound<'a> {
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut buf = Value::default();
        key.serialize(&mut buf)?;
        if self.key.is_none() {
            self.key = Some(buf);
        }
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut buf = Value::default();
        value.serialize(&mut buf)?;
        if self.key.is_some() {
            let key = self
                .key
                .take()
                .unwrap()
                .as_str()
                .map(|x| x.to_string())
                .unwrap();
            match self.ser {
                Value::Map(ref mut map) => {
                    map.insert(key, buf);
                }
                _ => {
                    let size = self.len.unwrap_or(0);
                    let mut map = Map::with_capacity(size);
                    map.insert(key, buf);
                    *self.ser = Value::from(map);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeStruct for SerializeCompound<'a> {
    type Ok = ();
    type Error = SerializeError;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut key_buf = Value::default();
        let mut value_buf = Value::default();
        key.serialize(&mut key_buf)?;
        value.serialize(&mut value_buf)?;
        let key = key_buf.as_str().map(|x| x.to_string()).unwrap();
        match self.ser {
            Value::Map(ref mut map) => {
                map.insert(key, value_buf);
            }
            _ => {
                let mut map = Map::new();
                map.insert(key, value_buf);
                *self.ser = Value::from(map);
            }
        }
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for SerializeTupleVariantImpl<'a> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut buf = Value::default();
        value.serialize(&mut buf)?;
        match self.seq {
            Value::Seq(ref mut seq) => {
                seq.push(buf);
            }
            _ => self.seq = default_seq(buf, Some(self.len)),
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = Map::new();
        map.insert(self.name.to_string(), self.seq);
        *self.ser = Value::from(map);
        Ok(())
    }
}

impl<'a> SerializeStructVariant for SerializeStructVariantImpl<'a> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut key_buf = Value::default();
        let mut value_buf = Value::default();
        key.serialize(&mut key_buf)?;
        value.serialize(&mut value_buf)?;
        let key = key_buf.as_str().map(|x| x.to_string()).unwrap();
        match self.map {
            Value::Map(ref mut map) => {
                map.insert(key, value_buf);
            }
            _ => {
                let mut map = Map::with_capacity(self.len);
                map.insert(key, value_buf);
                self.map = Value::from(map);
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = Map::new();
        map.insert(self.name.to_string(), self.map);
        *self.ser = Value::from(map);
        Ok(())
    }
}

impl<'a> SerializeTuple for SerializeCompound<'a> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a> SerializeTupleStruct for SerializeCompound<'a> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

fn default_seq(v: Value, len: Option<usize>) -> Value {
    let size = len.unwrap_or(0);
    let mut vec = Vec::with_capacity(size);
    vec.push(v);
    Value::from(vec)
}

fn default_empty_seq(len: Option<usize>) -> Value {
    let size = len.unwrap_or(0);
    let vec = Vec::<Value>::with_capacity(size);
    Value::from(vec)
}

fn default_empty_map(len: Option<usize>) -> Value {
    let size = len.unwrap_or(0);
    let map = Map::with_capacity(size);
    Value::from(map)
}

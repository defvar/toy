//! Part of query string.(Key or Value)
//!

use crate::error::QueryParseError;
use crate::ser_ops::NoSerialize;
use form_urlencoded::Target;
use toy_pack::ser::{Serializable, Serializer};

/// Trait for query string part.(Key or Value)
pub trait KeyOrValue {
    type Ok;

    fn serialize_static_str(self, value: &'static str) -> Result<Self::Ok, QueryParseError>;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, QueryParseError>;

    fn serialize_string(self, value: String) -> Result<Self::Ok, QueryParseError>;

    fn serialize_none(self) -> Result<Self::Ok, QueryParseError>;

    fn serialize_some<T: ?Sized + Serializable>(
        self,
        value: &T,
    ) -> Result<Self::Ok, QueryParseError>;
}

/// A Part
pub struct Part<KV> {
    inner: KV,
}

impl<KV> Part<KV> {
    pub fn new(inner: KV) -> Self {
        Self { inner }
    }
}

impl<KV> Part<KV>
where
    KV: KeyOrValue,
{
    fn into_integer<I>(self, value: I) -> Result<KV::Ok, QueryParseError>
    where
        I: itoa::Integer,
    {
        let mut buf = [b'\0'; 20];
        let len = itoa::write(&mut buf[..], value).unwrap();
        let part = unsafe { std::str::from_utf8_unchecked(&buf[0..len]) };
        Serializer::serialize_str(self, part)
    }

    fn into_floating<F>(self, value: F) -> Result<KV::Ok, QueryParseError>
    where
        F: ryu::Float,
    {
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);
        Serializer::serialize_str(self, part)
    }
}

pub struct Key;

impl KeyOrValue for Key {
    type Ok = String;

    fn serialize_static_str(self, value: &'static str) -> Result<Self::Ok, QueryParseError> {
        Ok(value.to_owned())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, QueryParseError> {
        Ok(value.to_owned())
    }

    fn serialize_string(self, value: String) -> Result<Self::Ok, QueryParseError> {
        Ok(value)
    }

    fn serialize_none(self) -> Result<Self::Ok, QueryParseError> {
        Err(QueryParseError::unsupported_key("None"))
    }

    fn serialize_some<T: ?Sized + Serializable>(
        self,
        _value: &T,
    ) -> Result<Self::Ok, QueryParseError> {
        Err(QueryParseError::unsupported_key("Some(T)"))
    }
}

pub struct Value<'out, 'i, 'key, Ta: Target> {
    inner: &'out mut form_urlencoded::Serializer<'i, Ta>,
    key: &'key str,
}

impl<'out, 'i, 'key, Ta> KeyOrValue for Value<'out, 'i, 'key, Ta>
where
    Ta: 'out + Target,
{
    type Ok = ();

    fn serialize_static_str(self, value: &'static str) -> Result<Self::Ok, QueryParseError> {
        self.serialize_str(value)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, QueryParseError> {
        self.inner.append_pair(self.key, value);
        Ok(())
    }

    fn serialize_string(self, value: String) -> Result<Self::Ok, QueryParseError> {
        self.serialize_str(&value)
    }

    fn serialize_none(self) -> Result<Self::Ok, QueryParseError> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serializable>(
        self,
        _value: &T,
    ) -> Result<Self::Ok, QueryParseError> {
        Err(QueryParseError::unsupported_value("Some(T)"))
    }
}

impl<'out, 'i, 'key, Ta> Value<'out, 'i, 'key, Ta>
where
    Ta: 'out + Target,
{
    pub fn new(inner: &'out mut form_urlencoded::Serializer<'i, Ta>, key: &'key str) -> Self {
        Self { inner, key }
    }
}

impl<KV> Serializer for Part<KV>
where
    KV: KeyOrValue,
{
    type Ok = KV::Ok;
    type Error = QueryParseError;
    type SeqAccessOps = NoSerialize<Self::Ok, Self::Error>;
    type MapAccessOps = NoSerialize<Self::Ok, Self::Error>;
    type StructAccessOps = NoSerialize<Self::Ok, Self::Error>;
    type TupleVariantOps = NoSerialize<Self::Ok, Self::Error>;
    type StructVariantOps = NoSerialize<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(if v { "true" } else { "false" })
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.into_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.into_floating(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.into_floating(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_string(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match std::str::from_utf8(v) {
            Ok(value) => self.inner.serialize_str(value),
            Err(e) => Err(QueryParseError::utf8_error(e)),
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SeqAccessOps, Self::Error> {
        Err(QueryParseError::unsupported("seq"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::MapAccessOps, Self::Error> {
        Err(QueryParseError::unsupported("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::StructAccessOps, Self::Error> {
        Err(QueryParseError::unsupported("struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_static_str(variant)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        Err(QueryParseError::unsupported("newtype variant"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::TupleVariantOps, Self::Error> {
        Err(QueryParseError::unsupported("tuple variant"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::StructVariantOps, Self::Error> {
        Err(QueryParseError::unsupported("struct variant"))
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        Err(QueryParseError::unsupported("Some(T)"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::unsupported("None"))
    }
}

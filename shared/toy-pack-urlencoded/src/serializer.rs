//! Serializer implementation.

use crate::error::QueryParseError;
use crate::ser_ops::{NoSerialize, SerializeCompound};
use form_urlencoded::Target;
use toy_pack::ser::{Serializable, Serializer};

pub struct Encoder<'out, 'i, T: Target> {
    inner: &'out mut form_urlencoded::Serializer<'i, T>,
}

impl<'out, 'i, T> Encoder<'out, 'i, T>
where
    T: 'out + Target,
{
    pub fn new(inner: &'out mut form_urlencoded::Serializer<'i, T>) -> Encoder<'out, 'i, T> {
        Self { inner }
    }
}

impl<'out, 'i, Ta> Serializer for Encoder<'out, 'i, Ta>
where
    Ta: 'out + Target,
{
    type Ok = &'out mut form_urlencoded::Serializer<'i, Ta>;
    type Error = QueryParseError;
    type SeqAccessOps = NoSerialize<Self::Ok, Self::Error>;
    type MapAccessOps = SerializeCompound<'out, 'i, Ta>;
    type StructAccessOps = SerializeCompound<'out, 'i, Ta>;
    type TupleVariantOps = NoSerialize<Self::Ok, Self::Error>;
    type StructVariantOps = NoSerialize<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SeqAccessOps, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::MapAccessOps, Self::Error> {
        Ok(SerializeCompound::new(self.inner))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::StructAccessOps, Self::Error> {
        Ok(SerializeCompound::new(self.inner))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(self.inner)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        value.serialize(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::TupleVariantOps, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::StructVariantOps, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        v.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.inner)
    }
}

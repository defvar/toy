use super::encode::{EncodeError, Encoder};
use crate::ser_ops::{SerializeCompound, SerializeTupleVariantImpl};
use core::num::FpCategory;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use serde::Serialize;
use std::io;

impl<'a, W> Serializer for &'a mut Encoder<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;
    type SerializeSeq = SerializeCompound<'a, W>;
    type SerializeTuple = SerializeCompound<'a, W>;
    type SerializeTupleStruct = SerializeCompound<'a, W>;
    type SerializeTupleVariant = SerializeTupleVariantImpl<'a, W>;
    type SerializeMap = SerializeCompound<'a, W>;
    type SerializeStruct = SerializeCompound<'a, W>;
    type SerializeStructVariant = SerializeCompound<'a, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_bool(v)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_i8(v)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_i16(v)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_i32(v)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_i64(v)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_u16(v)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_u32(v)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_u64(v)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => self.write_null(),
            _ => self.write_f32(v),
        }
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => self.write_null(),
            _ => self.write_f64(v),
        }
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_begin_string()?;
        self.write_string(v)?;
        self.write_end_string()?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            SerializeSeq::serialize_element(&mut seq, byte)?;
        }
        SerializeSeq::end(seq)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        v.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.write_null()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_begin_string()?;
        self.write_string(variant)?;
        self.write_end_string()?;
        Ok(())
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    #[inline]
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
        // { varinat_name: value }

        self.write_begin_object()?;

        self.write_begin_object_key(true)?;
        self.write_begin_string()?;
        self.write_string(variant)?;
        self.write_end_string()?;
        self.write_end_object_key()?;

        self.write_begin_object_value()?;
        value.serialize(&mut *self)?;
        self.write_end_object_value()?;

        self.write_end_object()?;
        Ok(())
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.write_begin_array()?;
        Ok(SerializeCompound::new(self))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // { varinat_name: [tuple] }

        self.write_begin_object()?;

        // key
        self.write_begin_object_key(true)?;
        self.write_begin_string()?;
        self.write_string(variant)?;
        self.write_end_string()?;
        self.write_end_object_key()?;

        // value
        self.write_begin_object_value()?;
        // inner array begin
        self.write_begin_array()?;
        Ok(SerializeTupleVariantImpl::new(self))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_begin_object()?;
        Ok(SerializeCompound::new(self))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_begin_object()?;
        Ok(SerializeCompound::new(self))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        // { varinat_name: { k: v, ... } }
        self.write_begin_object()?;

        // key (variant_name)
        self.write_begin_object_key(true)?;
        self.write_begin_string()?;
        self.write_string(variant)?;
        self.write_end_string()?;
        self.write_end_object_key()?;

        // value (variants)
        self.write_begin_object_value()?;

        self.write_begin_object()?;
        Ok(SerializeCompound::new(self))
    }
}

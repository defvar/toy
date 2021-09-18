use serde::ser::{Serialize, Serializer};

use super::encode::{EncodeError, Encoder, EncoderOps, Writer};
use super::ser_ops::{SerializeCompound, SerializeTupleVariantImpl};

impl<'a, W> Serializer for &'a mut Encoder<W>
where
    W: Writer,
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
        self.encode_bool(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.encode_sint(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.encode_uint(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.encode_f32(v)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.encode_f64(v)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.encode_str(v)
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.encode_bin(v)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.encode_nil().map_err(Into::into)
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.encode_nil()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.encode_array_len(0)?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_map_len(1)?;
        self.encode_str(variant)?;
        self.encode_nil().map_err(Into::into)
    }

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
        // map { key: variant, value: variant_value }
        self.encode_map_len(1)?;
        self.encode_str(variant)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = match len {
            Some(len) => len,
            None => return Err(EncodeError::unknown_seq_length()),
        };

        self.encode_array_len(len as u32)?;
        Ok(SerializeCompound::new(self))
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

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // map { key: variant, value: [variant_values_array] }
        self.encode_map_len(1)?;
        self.encode_str(variant)?;
        self.encode_array_len(len as u32)?;
        Ok(SerializeTupleVariantImpl::new(self))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = match len {
            Some(len) => len,
            None => return Err(EncodeError::unknown_seq_length()),
        };

        self.encode_map_len(len as u32)?;
        Ok(SerializeCompound::new(self))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.encode_map_len(len as u32)?;
        Ok(SerializeCompound::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _id: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        // map { key: variant, value: map { key: variant_field_name, value: variant_field_value } }
        self.encode_map_len(1)?;
        self.encode_str(variant)?;
        self.encode_map_len(len as u32)?;
        Ok(SerializeCompound::new(self))
    }
}

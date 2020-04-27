use toy_pack::ser::{Serializable, Serializer};

use super::encode::{EncodeError, Encoder, EncoderOps, Writer};
use crate::ser_ops::{SerializeCompound, SerializeTupleVariant};

impl<'a, W> Serializer for &'a mut Encoder<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;
    type SeqAccessOps = SerializeCompound<'a, W>;
    type MapAccessOps = SerializeCompound<'a, W>;
    type StructAccessOps = SerializeCompound<'a, W>;
    type TupleVariantOps = SerializeTupleVariant<'a, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.encode_bool(v)?;
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
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SeqAccessOps, Self::Error> {
        let len = match len {
            Some(len) => len,
            None => return Err(EncodeError::unknown_seq_length()),
        };

        self.encode_array_len(len as u32)?;
        Ok(SerializeCompound::new(self))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapAccessOps, Self::Error> {
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
    ) -> Result<Self::StructAccessOps, Self::Error> {
        self.encode_array_len(len as u32)?;
        Ok(SerializeCompound::new(self))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        idx: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_map_len(1)?;
        self.encode_u32(idx)?;
        self.encode_nil().map_err(Into::into)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        idx: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        // map { key: variant_idx, value: variant_value }
        self.encode_map_len(1)?;
        self.encode_u32(idx)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        idx: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::TupleVariantOps, Self::Error> {
        // map { key: variant_idx, value: [variant_values_array] }
        self.encode_map_len(1)?;
        self.encode_u32(idx)?;
        self.encode_array_len(len as u32)?;
        Ok(SerializeTupleVariant::new(self))
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        v.serialize(self)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.encode_nil().map_err(Into::into)
    }
}

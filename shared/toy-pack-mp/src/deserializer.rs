use toy_pack::deser::{Deserializer, Error, Visitor};

use super::decode::{DecodeError, Decoder, DecoderOps, Reader, Reference};
use super::deser_ops::DeserializeCompound;
use super::marker::Marker;

impl<'toy, 'a, B> Deserializer<'toy> for &'a mut Decoder<B>
where
    B: Reader<'toy>,
{
    type Error = DecodeError;

    fn deserialize_bool(self) -> Result<bool, Self::Error> {
        self.decode_bool()
    }

    #[inline]
    fn deserialize_u8(self) -> Result<u8, Self::Error> {
        self.decode_integer::<u8>()
    }

    #[inline]
    fn deserialize_u16(self) -> Result<u16, Self::Error> {
        self.decode_integer::<u16>()
    }

    #[inline]
    fn deserialize_u32(self) -> Result<u32, Self::Error> {
        self.decode_integer::<u32>()
    }

    #[inline]
    fn deserialize_u64(self) -> Result<u64, Self::Error> {
        self.decode_integer::<u64>()
    }

    #[inline]
    fn deserialize_i8(self) -> Result<i8, Self::Error> {
        self.decode_integer::<i8>()
    }

    #[inline]
    fn deserialize_i16(self) -> Result<i16, Self::Error> {
        self.decode_integer::<i16>()
    }

    #[inline]
    fn deserialize_i32(self) -> Result<i32, Self::Error> {
        self.decode_integer::<i32>()
    }

    #[inline]
    fn deserialize_i64(self) -> Result<i64, Self::Error> {
        self.decode_integer::<i64>()
    }

    #[inline]
    fn deserialize_f32(self) -> Result<f32, Self::Error> {
        self.decode_f32().map_err(Into::into)
    }

    #[inline]
    fn deserialize_f64(self) -> Result<f64, Self::Error> {
        self.decode_f64().map_err(Into::into)
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let s = self.decode_str()?;
        match s {
            Reference::Borrowed(b) => visitor.visit_borrowed_str(b),
            Reference::Copied(c) => visitor.visit_str(c),
        }
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let s = self.decode_bin()?;
        match s {
            Reference::Borrowed(b) => _visitor.visit_borrowed_bytes(b),
            Reference::Copied(c) => _visitor.visit_bytes(c),
        }
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let len = self.decode_array_len()?;
        visitor.visit_seq(DeserializeCompound::new(self, len as usize))
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let len = self.decode_map_len()?;
        visitor.visit_map(DeserializeCompound::new(self, len as usize))
    }

    #[inline]
    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let m = self.peek_marker()?;
        if m.is_array_type() {
            self.deserialize_seq(visitor)
        } else if m.is_map_type() {
            self.deserialize_map(visitor)
        } else {
            Err(Error::custom(
                "deserialize struct, must be a map type or array type.",
            ))
        }
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let len = self.decode_map_len()?;
        if len == 1 {
            visitor.visit_enum(DeserializeCompound::new(self, 0))
        } else {
            Err(Error::custom(format!(
                "Oops, map length:{}. When deserializing an enum from a 'map', length must be 1.",
                len
            )))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let marker = self.peek_marker()?;
        match marker {
            Marker::Nil => {
                let _ = self.get_marker()?; //discard marker
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let marker = self.peek_marker()?;

        if marker.is_array_type() {
            self.deserialize_seq(visitor)
        } else if marker.is_map_type() {
            self.deserialize_map(visitor)
        } else if marker.is_str_type() {
            self.deserialize_str(visitor)
        } else if marker.is_bin_type() {
            self.deserialize_bytes(visitor)
        } else {
            match marker {
                Marker::Nil => {
                    self.decode_nil()?;
                    visitor.visit_unit()
                }
                Marker::True | Marker::False => visitor.visit_bool(self.decode_bool()?),
                Marker::FixPos => visitor.visit_u8(self.decode_integer::<u8>()?),
                Marker::FixNeg => visitor.visit_i8(self.decode_integer::<i8>()?),
                Marker::U8 => visitor.visit_u8(self.decode_integer::<u8>()?),
                Marker::U16 => visitor.visit_u16(self.decode_integer::<u16>()?),
                Marker::U32 => visitor.visit_u32(self.decode_integer::<u32>()?),
                Marker::U64 => visitor.visit_u64(self.decode_integer::<u64>()?),
                Marker::I8 => visitor.visit_i8(self.decode_integer::<i8>()?),
                Marker::I16 => visitor.visit_i16(self.decode_integer::<i16>()?),
                Marker::I32 => visitor.visit_i32(self.decode_integer::<i32>()?),
                Marker::I64 => visitor.visit_i64(self.decode_integer::<i64>()?),
                Marker::Float32 => visitor.visit_f32(self.decode_integer::<f32>()?),
                Marker::Float64 => visitor.visit_f64(self.decode_integer::<f64>()?),
                Marker::FixExt1
                | Marker::FixExt2
                | Marker::FixExt4
                | Marker::FixExt8
                | Marker::FixExt16 => unimplemented!(),
                Marker::Ext8 | Marker::Ext16 | Marker::Ext32 => unimplemented!(),
                other => Err(DecodeError::from(other)),
            }
        }
    }

    #[inline]
    fn discard<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.discard_next()?;
        visitor.visit_none()
    }
}

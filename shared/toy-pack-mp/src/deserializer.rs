use toy_pack::deser::{Deserializer, Error, Visitor};

use super::decode::{DecodeError, Decoder, DecoderOps, Reader, Reference};
use super::deser_ops::DeserializeCompound;
use super::marker::Marker;

impl<'toy, 'a, B> Deserializer<'toy> for &'a mut Decoder<B>
    where B: Reader<'toy>
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
        where V: Visitor<'toy>
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'toy>
    {
        let s = self.decode_str()?;
        match s {
            Reference::Borrowed(b) => visitor.visit_borrowed_str(b),
            Reference::Copied(c) => visitor.visit_str(c),
        }
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'toy>
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'toy>
    {
        let len = self.decode_array_len()?;
        visitor.visit_seq(DeserializeCompound::new(self, len as usize))
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'toy> {
        let len = self.decode_map_len()?;
        visitor.visit_map(DeserializeCompound::new(self, len as usize))
    }

    #[inline]
    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'toy> {
        let m = self.peek_marker()?;
        if m.is_array_type() {
            self.deserialize_seq(visitor)
        } else if m.is_map_type() {
            self.deserialize_map(visitor)
        } else {
            Err(
                Error::custom("deserialize struct, must be a map type or array type.")
            )
        }
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'toy>
    {
        let len = self.decode_map_len()?;
        if len == 1 {
            visitor.visit_enum(DeserializeCompound::new(self, 0))
        } else {
            Err(
                Error::custom(format!("Oops, map length:{}. When deserializing an enum from a 'map', length must be 1.", len))
            )
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'toy>
    {
        let marker = self.peek_marker()?;
        match marker {
            Marker::Nil => {
                let _ = self.get_marker()?; //discard marker
                visitor.visit_none()
            }
            _ => {
                visitor.visit_some(self)
            }
        }
    }

    #[inline]
    fn discard<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'toy>
    {
        self.discard_next()?;
        visitor.visit_none()
    }
}

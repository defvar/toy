use toy_pack::deser::{Deserializer, Error, Visitor};

use super::decode::{DecodeError, Decoder, Reader, Reference};
use crate::decode::Token;
use crate::deser_ops::{DeserializeMap, DeserializeSeq, DeserializeVarinat};
use toy_pack::deser::from_primitive::FromPrimitive;

impl<'toy, 'a, B> Deserializer<'toy> for &'a mut Decoder<B>
where
    B: Reader<'toy>,
{
    type Error = DecodeError;

    fn deserialize_bool(self) -> Result<bool, Self::Error> {
        self.decode_bool()
    }

    fn deserialize_u8(self) -> Result<u8, Self::Error> {
        u8::from_u64(self.decode_u64()?).ok_or_else(|| Error::invalid_type("u8"))
    }

    fn deserialize_u16(self) -> Result<u16, Self::Error> {
        u16::from_u64(self.decode_u64()?).ok_or_else(|| Error::invalid_type("u16"))
    }

    fn deserialize_u32(self) -> Result<u32, Self::Error> {
        u32::from_u64(self.decode_u64()?).ok_or_else(|| Error::invalid_type("u32"))
    }

    fn deserialize_u64(self) -> Result<u64, Self::Error> {
        self.decode_u64()
    }

    fn deserialize_i8(self) -> Result<i8, Self::Error> {
        i8::from_i64(self.decode_i64()?).ok_or_else(|| Error::invalid_type("i8"))
    }

    fn deserialize_i16(self) -> Result<i16, Self::Error> {
        i16::from_i64(self.decode_i64()?).ok_or_else(|| Error::invalid_type("i16"))
    }

    fn deserialize_i32(self) -> Result<i32, Self::Error> {
        i32::from_i64(self.decode_i64()?).ok_or_else(|| Error::invalid_type("i32"))
    }

    fn deserialize_i64(self) -> Result<i64, Self::Error> {
        self.decode_i64()
    }

    fn deserialize_f32(self) -> Result<f32, Self::Error> {
        let f = self.decode_f64()?;
        Ok(f as f32)
    }

    fn deserialize_f64(self) -> Result<f64, Self::Error> {
        self.decode_f64()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let mut buf = Vec::new();
        let s = self.decode_str(&mut buf)?;
        match s {
            Reference::Borrowed(b) => visitor.visit_borrowed_str(b),
            Reference::Copied(c) => visitor.visit_str(c),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let b = self.peek()?;
        match b {
            Some(b'[') => {
                let _ = self.next()?;
                let ret = visitor.visit_seq(DeserializeSeq::new(self));

                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(_) => Err(DecodeError::invalid_type("array")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let b = self.peek()?;
        match b {
            Some(b'{') => {
                let _ = self.next()?;
                let ret = visitor.visit_map(DeserializeMap::new(self));

                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(_) => Err(DecodeError::invalid_type("map")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let b = self.peek()?;
        match b {
            Some(b'{') => {
                let _ = self.next()?;
                let ret = visitor.visit_map(DeserializeMap::new(self));

                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(b'[') => {
                let _ = self.next()?;
                let ret = visitor.visit_seq(DeserializeSeq::new(self));

                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(_) => Err(DecodeError::invalid_type("map or array")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.peek_until()? {
            Some(b'{') => {
                let value = visitor.visit_enum(DeserializeVarinat::new(self))?;
                match self.peek_until()? {
                    Some(b'}') => {
                        let _ = self.next()?;
                        Ok(value)
                    }
                    Some(_) => Err(DecodeError::error("ExpectedSomeValue")),
                    None => Err(DecodeError::eof_while_parsing_value()),
                }
            }
            Some(b'"') => visitor.visit_enum(DeserializeVarinat::new(self)),
            Some(_) => Err(DecodeError::error("ExpectedSomeValue")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.peek_until()? {
            Some(b'n') => {
                let _ = self.next()?;
                self.parse_ident(b"ull")?;
                visitor.visit_none()
            }
            Some(_) => visitor.visit_some(self),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        unimplemented!()
    }

    fn discard<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        unimplemented!()
    }
}

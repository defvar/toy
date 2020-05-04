use toy_pack::deser::{Deserializer, Error, FromPrimitive, Visitor};

use super::decode::{DecodeError, Decoder, Reader, Reference};
use crate::decode::Token;
use crate::deser_ops::{
    DeserializeMap, DeserializeSeq, DeserializeUnitVarinat, DeserializeVarinat,
};
use crate::ParseNumber;

macro_rules! de_number {
    ($t: ident, $func: ident, $expected: literal) => {
       fn $func(self) -> Result<$t, Self::Error> {
           match self.decode_number()? {
               ParseNumber::U64(v) => $t::from_u64(v),
               ParseNumber::I64(v) => $t::from_i64(v),
               ParseNumber::F64(v) => Some(v as $t),
           }
           .ok_or_else(|| Error::invalid_type($expected))
        }
    };
}

impl<'toy, 'a, B> Deserializer<'toy> for &'a mut Decoder<B>
where
    B: Reader<'toy>,
{
    type Error = DecodeError;

    fn deserialize_bool(self) -> Result<bool, Self::Error> {
        self.decode_bool()
    }

    de_number!(u8, deserialize_u8, "u8");
    de_number!(u16, deserialize_u16, "u16");
    de_number!(u32, deserialize_u32, "u32");
    de_number!(u64, deserialize_u64, "u64");

    de_number!(i8, deserialize_i8, "i8");
    de_number!(i16, deserialize_i16, "i16");
    de_number!(i32, deserialize_i32, "i32");
    de_number!(i64, deserialize_i64, "i64");

    de_number!(f32, deserialize_f32, "f32");
    de_number!(f64, deserialize_f64, "f64");

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
        let b = self.peek_token()?;
        match b {
            Some(Token::BeginArray) => {
                self.consume();
                let ret = visitor.visit_seq(DeserializeSeq::new(self));

                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(other) => Err(DecodeError::invalid_token(other, "BeginArray")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let b = self.peek_token()?;
        match b {
            Some(Token::BeginObject) => {
                self.consume();
                let ret = visitor.visit_map(DeserializeMap::new(self));

                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(other) => Err(DecodeError::invalid_token(other, "BeginObject")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let t = self.peek_token()?;
        match t {
            Some(Token::BeginObject) => {
                self.consume();
                let ret = visitor.visit_map(DeserializeMap::new(self));

                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(Token::BeginArray) => {
                self.consume();
                let ret = visitor.visit_seq(DeserializeSeq::new(self));

                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(other) => Err(DecodeError::invalid_token(
                other,
                "BeginObject or BeginArray",
            )),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.peek_token()? {
            Some(Token::BeginObject) => {
                self.consume();
                let value = visitor.visit_enum(DeserializeVarinat::new(self))?;
                match self.peek_token()? {
                    Some(Token::EndObject) => {
                        self.consume();
                        Ok(value)
                    }
                    Some(_) => Err(DecodeError::error("ExpectedSomeValue")),
                    None => Err(DecodeError::eof_while_parsing_value()),
                }
            }
            Some(Token::String) => visitor.visit_enum(DeserializeUnitVarinat::new(self)),
            Some(_) => Err(DecodeError::error("ExpectedSomeValue")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.peek_token()? {
            Some(Token::Null) => {
                self.consume();
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
        match self.peek_token()? {
            Some(Token::True) | Some(Token::False) => visitor.visit_bool(self.decode_bool()?),
            Some(Token::Number) => match self.decode_number()? {
                ParseNumber::U64(v) => visitor.visit_u64(v),
                ParseNumber::I64(v) => visitor.visit_i64(v),
                ParseNumber::F64(v) => visitor.visit_f64(v),
            },
            Some(Token::String) => {
                let mut buf = Vec::new();
                let s = self.decode_str(&mut buf)?;
                match s {
                    Reference::Borrowed(b) => visitor.visit_borrowed_str(b),
                    Reference::Copied(c) => visitor.visit_str(c),
                }
            }
            Some(Token::BeginArray) => {
                self.consume();
                let ret = visitor.visit_seq(DeserializeSeq::new(self));

                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(Token::BeginObject) => {
                self.consume();
                let ret = visitor.visit_map(DeserializeMap::new(self));

                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            Some(_) => Err(DecodeError::error("ExpectedSomeValue")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn discard<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let mut array_depth = 0;
        let mut object_depth = 0;
        let mut buf = Vec::new();

        loop {
            match self.peek_token()? {
                Some(Token::True) | Some(Token::False) => {
                    let _ = self.decode_bool()?;
                }
                Some(Token::Number) => {
                    let _ = self.decode_number()?;
                }
                Some(Token::String) => {
                    let _ = self.decode_str(&mut buf)?;
                }
                Some(Token::BeginArray) => {
                    array_depth += 1;
                }
                Some(Token::EndArray) => {
                    array_depth -= 1;
                }
                Some(Token::BeginObject) => {
                    object_depth += 1;
                }
                Some(Token::EndObject) => {
                    object_depth -= 1;
                }
                Some(_) => (),
                None => {
                    if array_depth > 0 || object_depth > 0 {
                        return Err(DecodeError::eof_while_parsing_value());
                    }
                }
            }
            if array_depth == 0 && object_depth == 0 {
                break;
            }
        }
        visitor.visit_none()
    }
}

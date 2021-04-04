use toy_pack::deser::{
    DeserializableCore, DeserializeMapOps, DeserializeSeqOps, DeserializeVariantOps, Visitor,
};

use crate::decode::{Reference, Token};

use super::decode::{DecodeError, Decoder, Reader};

pub struct DeserializeSeq<'a, B: 'a> {
    de: &'a mut Decoder<B>,
    first: bool,
}

impl<'a, B: 'a> DeserializeSeq<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de, first: true }
    }
}

impl<'toy, 'a, B> DeserializeSeqOps<'toy> for DeserializeSeq<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_core<T>(&mut self, core: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        let b = match self.de.peek_until()? {
            Some(b']') => return Ok(None),
            Some(b',') if !self.first => {
                self.de.consume();
                self.de.peek_until()?
            }
            Some(b) => {
                if self.first {
                    self.first = false;
                    Some(b)
                } else {
                    return Err(DecodeError::array_comma_or_end(self.de.position()));
                }
            }
            None => return Err(DecodeError::eof_while_parsing_value()),
        };

        match b {
            Some(b']') => Err(DecodeError::trailing_comma(self.de.position())),
            Some(_) => core.deserialize(&mut *self.de).map(Some),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

pub struct DeserializeMap<'a, B: 'a> {
    de: &'a mut Decoder<B>,
    first: bool,
}

impl<'a, B: 'a> DeserializeMap<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de, first: true }
    }
}

impl<'toy, 'a, B> DeserializeMapOps<'toy> for DeserializeMap<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let b = match self.de.peek_until()? {
            Some(b'}') => return Ok(None),
            Some(b',') if !self.first => {
                self.de.consume();
                self.de.peek_until()?
            }
            Some(b) => {
                if self.first {
                    self.first = false;
                    Some(b)
                } else {
                    return Err(DecodeError::object_comma_or_end(self.de.position()));
                }
            }
            None => return Err(DecodeError::eof_while_parsing_value()),
        };

        match b {
            Some(b'}') => Err(DecodeError::trailing_comma(self.de.position())),
            Some(b'"') => {
                let mut buf = Vec::new();
                match self.de.decode_str(&mut buf)? {
                    Reference::Borrowed(b) => visitor.visit_borrowed_str::<DecodeError>(b),
                    Reference::Copied(b) => visitor.visit_str::<DecodeError>(b),
                }
                .map(Some)
            }
            Some(_) => Err(DecodeError::key_must_be_a_string(self.de.position())),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn next_key_core<T>(&mut self, core: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        let b = match self.de.peek_token()? {
            Some(Token::EndObject) => return Ok(None),
            Some(Token::Comma) if !self.first => {
                self.de.consume();
                self.de.peek_token()?
            }
            Some(b) => {
                if self.first {
                    self.first = false;
                    Some(b)
                } else {
                    return Err(DecodeError::object_comma_or_end(self.de.position()));
                }
            }
            None => return Err(DecodeError::eof_while_parsing_value()),
        };

        match b {
            Some(Token::EndObject) => Err(DecodeError::trailing_comma(self.de.position())),
            Some(Token::String) => core.deserialize(&mut *self.de).map(Some),
            Some(_) => Err(DecodeError::key_must_be_a_string(self.de.position())),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn next_value_core<T>(&mut self, core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        match self.de.peek_token()? {
            Some(Token::Colon) => {
                self.de.consume();
                core.deserialize(&mut *self.de)
            }
            Some(_) => Err(DecodeError::expected_colon(self.de.position())),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

pub struct DeserializeVarinat<'a, B: 'a> {
    de: &'a mut Decoder<B>,
}

impl<'a, B: 'a> DeserializeVarinat<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de }
    }
}

impl<'toy, 'a, B> DeserializeVariantOps<'toy> for DeserializeVarinat<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn variant_identifier<V>(self, visitor: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        let mut buf = Vec::new();
        let s = match self.de.decode_str(&mut buf)? {
            Reference::Borrowed(b) => visitor.visit_borrowed_str::<DecodeError>(b),
            Reference::Copied(c) => visitor.visit_str::<DecodeError>(c),
        }?;
        match self.de.peek_token()? {
            Some(Token::Colon) => {
                self.de.consume();
            }
            Some(_) => return Err(DecodeError::expected_colon(self.de.position())),
            None => return Err(DecodeError::eof_while_parsing_value()),
        }
        Ok((s, self))
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_core<T>(self, core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        core.deserialize(self.de)
    }

    fn tuple_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        toy_pack::deser::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        toy_pack::deser::Deserializer::deserialize_struct(self.de, visitor)
    }
}

pub struct DeserializeUnitVarinat<'a, B: 'a> {
    de: &'a mut Decoder<B>,
}

impl<'a, B: 'a> DeserializeUnitVarinat<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de }
    }
}

impl<'toy, 'a, B> DeserializeVariantOps<'toy> for DeserializeUnitVarinat<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn variant_identifier<V>(
        self,
        visitor: V,
    ) -> Result<(<V as Visitor<'toy>>::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        let mut buf = Vec::new();
        let s = match self.de.decode_str(&mut buf)? {
            Reference::Borrowed(b) => visitor.visit_borrowed_str::<DecodeError>(b),
            Reference::Copied(c) => visitor.visit_str::<DecodeError>(c),
        }?;
        Ok((s, self))
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_core<T>(
        self,
        _core: T,
    ) -> Result<<T as DeserializableCore<'toy>>::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        unreachable!()
    }

    fn tuple_variant<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        unreachable!()
    }

    fn struct_variant<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        unreachable!()
    }
}

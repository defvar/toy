use super::decode::{DecodeError, Decoder, Reader};
use crate::decode::Token;
use serde::de::{
    DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor,
};

pub struct DeserializeSeq<'a, B: 'a> {
    de: &'a mut Decoder<B>,
    first: bool,
}

impl<'a, B: 'a> DeserializeSeq<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de, first: true }
    }
}

impl<'toy, 'a, B> SeqAccess<'toy> for DeserializeSeq<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'toy>,
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
            Some(_) => seed.deserialize(&mut *self.de).map(Some),
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

impl<'toy, 'a, B> MapAccess<'toy> for DeserializeMap<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'toy>,
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
            Some(Token::String) => seed.deserialize(&mut *self.de).map(Some),
            Some(_) => Err(DecodeError::key_must_be_a_string(self.de.position())),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'toy>,
    {
        match self.de.peek_token()? {
            Some(Token::Colon) => {
                self.de.consume();
                seed.deserialize(&mut *self.de)
            }
            Some(_) => Err(DecodeError::expected_colon(self.de.position())),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

pub struct DeserializeVariant<'a, B: 'a> {
    de: &'a mut Decoder<B>,
}

pub struct DeserializeUnitVariant<'a, B: 'a> {
    de: &'a mut Decoder<B>,
}

impl<'a, B: 'a> DeserializeVariant<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de }
    }
}

impl<'a, B: 'a> DeserializeUnitVariant<'a, B> {
    pub fn new(de: &'a mut Decoder<B>) -> Self {
        Self { de }
    }
}

impl<'toy, 'a, B> VariantAccess<'toy> for DeserializeVariant<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        serde::de::Deserialize::deserialize(self.de)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        serde::de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        serde::de::Deserializer::deserialize_struct(self.de, "", _fields, visitor)
    }
}

impl<'toy, 'a, B> EnumAccess<'toy> for DeserializeVariant<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'toy>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        match self.de.peek_token()? {
            Some(Token::Colon) => {
                self.de.consume();
            }
            Some(_) => return Err(DecodeError::expected_colon(self.de.position())),
            None => return Err(DecodeError::eof_while_parsing_value()),
        };
        Ok((val, self))
    }
}

impl<'toy, 'a, B> VariantAccess<'toy> for DeserializeUnitVariant<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(serde::de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

impl<'toy, 'a, B> EnumAccess<'toy> for DeserializeUnitVariant<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'toy>,
    {
        let v = seed.deserialize(&mut *self.de)?;
        Ok((v, self))
    }
}

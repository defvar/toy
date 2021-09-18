use super::decode::{DecodeError, Decoder, DecoderOps, Reader};
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess};

/// Any Deserialize Ops implementation for MessagePack.
///
/// AccessOpsのMessagePack実装
///
pub struct DeserializeCompound<'a, B: 'a> {
    de: &'a mut Decoder<B>,
    remaining: usize,
}

impl<'a, B: 'a> DeserializeCompound<'a, B> {
    pub fn new(de: &'a mut Decoder<B>, size: usize) -> Self {
        Self {
            de,
            remaining: size,
        }
    }
}

impl<'toy, 'a, B> SeqAccess<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            if self.de.remaining() > 0 {
                seed.deserialize(&mut *self.de).map(Some)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

impl<'toy, 'a, B> MapAccess<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            if self.de.remaining() > 0 {
                seed.deserialize(&mut *self.de).map(Some)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'toy>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

impl<'toy, 'a, B> EnumAccess<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'toy>,
    {
        Ok((seed.deserialize(&mut *self.de)?, self))
    }
}

impl<'toy, 'a, B> VariantAccess<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.de.decode_nil()?;
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'toy>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'toy>,
    {
        serde::de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'toy>,
    {
        serde::de::Deserializer::deserialize_map(self.de, visitor)
    }
}

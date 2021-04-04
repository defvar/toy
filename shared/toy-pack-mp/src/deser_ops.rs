use toy_pack::deser::{
    DeserializableCore, DeserializeMapOps, DeserializeSeqOps, DeserializeVariantOps, Visitor,
};

use crate::decode::Reference;

use super::decode::{DecodeError, Decoder, DecoderOps, Reader};

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

impl<'toy, 'a, B> DeserializeSeqOps<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_core<T>(&mut self, core: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            core.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

impl<'toy, 'a, B> DeserializeMapOps<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: Visitor<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;

            match self.de.decode_str()? {
                Reference::Borrowed(b) => visitor.visit_borrowed_str::<DecodeError>(b),
                Reference::Copied(b) => visitor.visit_str::<DecodeError>(b),
            }
            .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_key_core<T>(&mut self, core: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        if self.remaining > 0 {
            self.remaining -= 1;
            core.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_core<T>(&mut self, core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        core.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

impl<'toy, 'a, B> DeserializeVariantOps<'toy> for DeserializeCompound<'a, B>
where
    B: Reader<'toy> + 'a,
{
    type Error = DecodeError;

    fn variant_identifier<V>(self, visitor: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        Ok((
            visitor.visit_u32::<DecodeError>(self.de.decode_u32()?)?,
            self,
        ))
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.de.decode_nil()?;
        Ok(())
    }

    fn newtype_variant_core<T>(self, core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        core.deserialize(&mut *self.de)
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

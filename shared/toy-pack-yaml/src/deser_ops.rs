use toy_pack::deser::{
    DeserializableCore, DeserializeMapOps, DeserializeSeqOps, DeserializeVariantOps, Visitor,
};

use super::decoder::{Decoder, Event};
use super::error::YamlError;

/// Any Deserialize Ops implementation for YAML.
///
/// Deserialize OpsのYAML実装
///
pub struct DeserializeCompound<'a> {
    de: &'a mut Decoder,
}

impl<'a> DeserializeCompound<'a> {
    pub fn new(de: &'a mut Decoder) -> Self {
        Self { de }
    }
}

impl<'toy, 'a> DeserializeSeqOps<'toy> for DeserializeCompound<'a> {
    type Error = YamlError;

    fn next_core<T>(&mut self, core: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        match *self.de.peek()?.0 {
            Event::SequenceEnd => {
                self.de.next()?; //consume
                Ok(None)
            }
            _ => core.deserialize(&mut *self.de).map(Some),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<'toy, 'a> DeserializeMapOps<'toy> for DeserializeCompound<'a> {
    type Error = YamlError;

    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match *self.de.peek()?.0 {
            Event::MappingEnd => {
                self.de.next()?; //consume
                Ok(None)
            }
            _ => self
                .de
                .decode_string()
                .map(|x| visitor.visit_str(x.as_str()))?
                .map(Some),
        }
    }

    fn next_key_core<T>(&mut self, core: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        match *self.de.peek()?.0 {
            Event::MappingEnd => {
                self.de.next()?; //consume
                Ok(None)
            }
            _ => core.deserialize(&mut *self.de).map(Some),
        }
    }

    fn next_value_core<T>(&mut self, core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        core.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<'toy, 'a> DeserializeVariantOps<'toy> for DeserializeCompound<'a> {
    type Error = YamlError;

    fn variant_identifier<V>(self, visitor: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        match *self.de.peek()?.0 {
            Event::MappingStart => {
                self.de.next()?; //consume
            }
            _ => (),
        };
        Ok((
            visitor.visit_str::<YamlError>(self.de.decode_string()?.as_str())?,
            self,
        ))
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        //self.de.decode_null()?;
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

    fn struct_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        toy_pack::deser::Deserializer::deserialize_map(self.de, visitor)
    }
}

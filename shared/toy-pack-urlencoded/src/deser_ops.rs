use crate::deserializer::Part;
use crate::error::QueryParseError;
use core::marker::PhantomData;
use std::borrow::Cow;
use toy_pack::deser::{
    DeserializableCore, DeserializeMapOps, DeserializeVariantOps, Error, Visitor,
};

pub struct DeserializeMap<'a, I>
where
    I: Iterator<Item = (Cow<'a, str>, Cow<'a, str>)>,
{
    iter: I,
    value: Option<Cow<'a, str>>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, I> DeserializeMap<'a, I>
where
    I: Iterator<Item = (Cow<'a, str>, Cow<'a, str>)>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            value: None,
            lifetime: PhantomData,
        }
    }
}

impl<'toy, I> DeserializeMapOps<'toy> for DeserializeMap<'toy, I>
where
    I: Iterator<Item = (Cow<'toy, str>, Cow<'toy, str>)>,
{
    type Error = QueryParseError;

    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                match k {
                    Cow::Borrowed(value) => visitor.visit_borrowed_str(value).map(Some),
                    Cow::Owned(value) => visitor.visit_string(value).map(Some),
                }
            }
            None => Ok(None),
        }
    }

    fn next_key_core<T>(&mut self, element: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                element.deserialize(Part(k)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_core<T>(&mut self, element: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        element.deserialize(Part(value))
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

pub struct DeserializeVariant<'toy>(pub(crate) Cow<'toy, str>);

impl<'toy> DeserializeVariantOps<'toy> for DeserializeVariant<'toy> {
    type Error = QueryParseError;

    fn variant_identifier<V>(self, visitor: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        let s = self.0.clone();
        Ok((
            match self.0 {
                Cow::Borrowed(value) => visitor.visit_borrowed_str::<QueryParseError>(value),
                Cow::Owned(value) => visitor.visit_string::<QueryParseError>(value),
            }?,
            DeserializeVariant(s),
        ))
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_core<T>(self, _core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn tuple_variant<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(Error::custom("expected unit variant"))
    }
}

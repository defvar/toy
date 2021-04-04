use crate::error::QueryParseError;
use crate::part::{Key, Part, Value};
use core::marker::PhantomData;
use form_urlencoded::Target;
use std::borrow::Cow;
use toy_pack::ser::{
    Serializable, SerializeMapOps, SerializeSeqOps, SerializeStructOps, SerializeStructVariantOps,
    SerializeTupleVariantOps,
};

pub struct SerializeCompound<'o, 'i, Ta: Target> {
    inner: &'o mut form_urlencoded::Serializer<'i, Ta>,
    key: Option<Cow<'static, str>>,
}

pub struct NoSerialize<Ok, Err> {
    _t: PhantomData<(Ok, Err)>,
}

impl<'out, 'i, Ta> SerializeCompound<'out, 'i, Ta>
where
    Ta: 'out + Target,
{
    pub fn new(inner: &'out mut form_urlencoded::Serializer<'i, Ta>) -> Self {
        Self { inner, key: None }
    }
}

impl<'out, 'i, Ta> SerializeMapOps for SerializeCompound<'out, 'i, Ta>
where
    Ta: 'out + Target,
{
    type Ok = &'out mut form_urlencoded::Serializer<'i, Ta>;
    type Error = QueryParseError;

    fn next_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        let k = key.serialize(Part::new(Key))?;
        self.key = Some(Cow::from(k));
        Ok(())
    }

    fn next_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        if let Some(k) = self.key.as_ref() {
            value.serialize(Part::new(Value::new(self.inner, k.as_ref())))?;
        }
        self.key = None;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.inner)
    }
}

impl<'out, 'i, Ta> SerializeStructOps for SerializeCompound<'out, 'i, Ta>
where
    Ta: 'out + Target,
{
    type Ok = &'out mut form_urlencoded::Serializer<'i, Ta>;
    type Error = QueryParseError;

    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        let k = key.serialize(Part::new(Key))?;
        value.serialize(Part::new(Value::new(self.inner, k.as_ref())))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.inner)
    }
}

//////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

impl<Ok, E> SerializeSeqOps for NoSerialize<Ok, E>
where
    E: toy_pack::ser::Error,
{
    type Ok = Ok;
    type Error = E;

    fn next<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        Err(toy_pack::ser::Error::custom("not support"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(toy_pack::ser::Error::custom("not support"))
    }
}

impl<Ok, E> SerializeMapOps for NoSerialize<Ok, E>
where
    E: toy_pack::ser::Error,
{
    type Ok = Ok;
    type Error = E;

    fn next_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        Err(toy_pack::ser::Error::custom("not support"))
    }

    fn next_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        Err(toy_pack::ser::Error::custom("not support"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(toy_pack::ser::Error::custom("not support"))
    }
}

impl<Ok, E> SerializeTupleVariantOps for NoSerialize<Ok, E>
where
    E: toy_pack::ser::Error,
{
    type Ok = Ok;
    type Error = E;

    fn next<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        Err(toy_pack::ser::Error::custom("not support"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(toy_pack::ser::Error::custom("not support"))
    }
}

impl<Ok, E> SerializeStructOps for NoSerialize<Ok, E>
where
    E: toy_pack::ser::Error,
{
    type Ok = Ok;
    type Error = E;

    fn field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        Err(toy_pack::ser::Error::custom("not support"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(toy_pack::ser::Error::custom("not support"))
    }
}

impl<Ok, E> SerializeStructVariantOps for NoSerialize<Ok, E>
where
    E: toy_pack::ser::Error,
{
    type Ok = Ok;
    type Error = E;

    fn field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        Err(toy_pack::ser::Error::custom("not support"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(toy_pack::ser::Error::custom("not support"))
    }
}

use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};

use super::encode::{EncodeError, Encoder, Writer};

/// Any Serialize Ops implementation MessagePack.
///
/// AccessOpsのMessagePack実装
///
pub struct SerializeCompound<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
}

pub struct SerializeTupleVariantImpl<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
}

impl<'a, W> SerializeCompound<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser }
    }
}

impl<'a, W> SerializeTupleVariantImpl<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser }
    }
}

impl<'a, W> SerializeSeq for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeMap for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut *self.ser)
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStruct for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeTupleVariant for SerializeTupleVariantImpl<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStructVariant for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeTuple for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeTupleStruct for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

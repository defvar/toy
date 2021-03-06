use toy_pack::ser::{
    Serializable, SerializeMapOps, SerializeSeqOps, SerializeStructOps, SerializeStructVariantOps,
    SerializeTupleVariantOps,
};

use super::encode::{EncodeError, Encoder, Writer};

/// Any Serialize Ops implementation MessagePack.
///
/// AccessOpsのMessagePack実装
///
pub struct SerializeCompound<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
}

pub struct SerializeTupleVariant<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
}

impl<'a, W> SerializeCompound<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser }
    }
}

impl<'a, W> SerializeTupleVariant<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser }
    }
}

impl<'a, W> SerializeSeqOps for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn next<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeMapOps for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn next_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        key.serialize(&mut *self.ser)
    }

    #[inline]
    fn next_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStructOps for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeTupleVariantOps for SerializeTupleVariant<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    fn next<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStructVariantOps for SerializeCompound<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = EncodeError;

    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

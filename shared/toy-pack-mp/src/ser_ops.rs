use toy_pack::ser::{Serializable, SerializeMapOps, SerializeSeqOps, SerializeStructOps};

use super::encode::{EncodeError, Encoder, Writer};

/// Any Serialize Ops implementation MessagePack.
///
/// AccessOpsのMessagePack実装
///
pub struct SerializeCompound<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
}

impl<'a, W> SerializeCompound<'a, W> {
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
    fn next<T>(&mut self, value: &T) -> Result<(), Self::Error>
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
    fn next_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        key.serialize(&mut *self.ser)
    }

    #[inline]
    fn next_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
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
    fn field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
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

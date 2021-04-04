use super::encode::{EncodeError, Encoder};
use std::io;
use toy_pack::ser::{
    Serializable, SerializeMapOps, SerializeSeqOps, SerializeStructOps, SerializeStructVariantOps,
    SerializeTupleVariantOps,
};

/// Any Serialize Ops implementation json.
///
pub struct SerializeCompound<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
    first: bool,
}

pub struct SerializeTupleVariant<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
    first: bool,
}

impl<'a, W> SerializeCompound<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser, first: true }
    }
}

impl<'a, W> SerializeTupleVariant<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser, first: true }
    }
}

impl<'a, W> SerializeSeqOps for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn next<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.ser.write_begin_array_element(self.first)?;
        self.first = false;
        value.serialize(&mut *self.ser)?;
        self.ser.write_end_array_element()?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_array()?;
        Ok(())
    }
}

impl<'a, W> SerializeMapOps for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn next_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.ser.write_begin_object_key(self.first)?;
        self.first = false;
        key.serialize(&mut *self.ser)?;
        self.ser.write_end_object_key()?;
        Ok(())
    }

    #[inline]
    fn next_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.ser.write_begin_object_value()?;
        value.serialize(&mut *self.ser)?;
        self.ser.write_end_object_value()?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_object()?;
        Ok(())
    }
}

impl<'a, W> SerializeStructOps for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        SerializeMapOps::next_key(self, key)?;
        SerializeMapOps::next_value(self, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMapOps::end(self)?;
        Ok(())
    }
}

impl<'a, W> SerializeTupleVariantOps for SerializeTupleVariant<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn next<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.ser.write_begin_array_element(self.first)?;
        self.first = false;
        value.serialize(&mut *self.ser)?;
        self.ser.write_end_array_element()?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_array()?;
        self.ser.write_end_object_value()?;
        self.ser.write_end_object()?;
        Ok(())
    }
}

impl<'a, W> SerializeStructVariantOps for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        SerializeMapOps::next_key(self, key)?;
        SerializeMapOps::next_value(self, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_object()?;
        self.ser.write_end_object_value()?;
        self.ser.write_end_object()?;
        Ok(())
    }
}

use super::encode::{EncodeError, Encoder};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serialize;
use std::io;

/// Any Serialize Ops implementation json.
///
pub struct SerializeCompound<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
    first: bool,
}

pub struct SerializeTupleVariantImpl<'a, W: 'a> {
    ser: &'a mut Encoder<W>,
    first: bool,
}

impl<'a, W> SerializeCompound<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser, first: true }
    }
}

impl<'a, W> SerializeTupleVariantImpl<'a, W> {
    pub fn new(ser: &'a mut Encoder<W>) -> Self {
        Self { ser, first: true }
    }
}

impl<'a, W> SerializeSeq for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
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

impl<'a, W> SerializeMap for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser.write_begin_object_key(self.first)?;
        self.first = false;
        key.serialize(&mut *self.ser)?;
        self.ser.write_end_object_key()?;
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
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

impl<'a, W> SerializeStruct for SerializeCompound<'a, W>
where
    W: io::Write,
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
        SerializeMap::serialize_key(self, key)?;
        SerializeMap::serialize_value(self, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)?;
        Ok(())
    }
}

impl<'a, W> SerializeTuple for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a, W> SerializeTupleVariant for SerializeTupleVariantImpl<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
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

impl<'a, W> SerializeTupleStruct for SerializeCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a, W> SerializeStructVariant for SerializeCompound<'a, W>
where
    W: io::Write,
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
        SerializeMap::serialize_key(self, key)?;
        SerializeMap::serialize_value(self, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_object()?;
        self.ser.write_end_object_value()?;
        self.ser.write_end_object()?;
        Ok(())
    }
}

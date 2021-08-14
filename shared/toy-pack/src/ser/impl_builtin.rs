use std::path::{Path, PathBuf};

use super::{Error, Serializable, Serializer};
use crate::ser::SerializeStructOps;
use core::time::Duration;
use std::borrow::Cow;

impl Serializable for () {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit()
    }
}

///////////////////////////////////////////////////

impl<'a, T: ?Sized> Serializable for Cow<'a, T>
where
    T: Serializable + ToOwned,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

///////////////////////////////////////////////////

impl<T> Serializable for Option<T>
where
    T: Serializable,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Some(ref v) => serializer.serialize_some(v),
            None => serializer.serialize_none(),
        }
    }
}

///////////////////////////////////////////////////

impl<T: ?Sized> Serializable for Box<T>
where
    T: Serializable,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

///////////////////////////////////////////////////

impl Serializable for Path {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.to_str() {
            Some(v) => v.serialize(serializer),
            None => Err(Error::custom("invalid UTF-8 characters")),
        }
    }
}

///////////////////////////////////////////////////

impl Serializable for PathBuf {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_path().serialize(serializer)
    }
}

///////////////////////////////////////////////////

impl Serializable for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("Duration", 2)?;
        ser.field("secs", &self.as_secs())?;
        ser.field("nanos", &self.subsec_nanos())?;
        ser.end()
    }
}

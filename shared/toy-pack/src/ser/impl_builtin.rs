use std::path::{Path, PathBuf};

use super::{Error, Serializable, Serializer};

///////////////////////////////////////////////////

impl<T> Serializable for Option<T> where T: Serializable {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match *self {
            Some(ref v) => serializer.serialize_some(v),
            None => serializer.serialize_none(),
        }
    }
}


///////////////////////////////////////////////////

impl Serializable for Path {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
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
        where S: Serializer,
    {
        self.as_path().serialize(serializer)
    }
}

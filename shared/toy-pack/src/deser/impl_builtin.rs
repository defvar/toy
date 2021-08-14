use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::{Deserializable, DeserializableCore, Deserializer, Error, Visitor};
use crate::deser::discard::Discard;
use crate::deser::{DeserializeMapOps, DeserializeSeqOps};
use core::time::Duration;
use std::borrow::Cow;

struct UnitVisitor;

impl<'toy> Visitor<'toy> for UnitVisitor {
    type Value = ();

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(())
    }
}

impl<'toy> Deserializable<'toy> for () {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_unit(UnitVisitor)
    }
}

///////////////////////////////////////////////////

impl<'toy, 'a, T: ?Sized> Deserializable<'toy> for Cow<'a, T>
where
    T: ToOwned,
    T::Owned: Deserializable<'toy>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        T::Owned::deserialize(deserializer).map(Cow::Owned)
    }
}

///////////////////////////////////////////////////

impl<'toy, T> DeserializableCore<'toy> for PhantomData<T>
where
    T: Deserializable<'toy>,
{
    type Value = T;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'toy>,
    {
        T::deserialize(deserializer)
    }
}

///////////////////////////////////////////////////

impl<'toy, T> Deserializable<'toy> for Box<T>
where
    T: Deserializable<'toy>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        T::deserialize(deserializer).map(Box::new)
    }
}

///////////////////////////////////////////////////

struct OptionVisitor<T> {
    marker: PhantomData<T>,
}

impl<'a, T> Visitor<'a> for OptionVisitor<T>
where
    T: Deserializable<'a>,
{
    type Value = Option<T>;

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'a>,
    {
        T::deserialize(deserializer).map(Some)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }
}

impl<'toy, T> Deserializable<'toy> for Option<T>
where
    T: Deserializable<'toy>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_option(OptionVisitor::<T> {
            marker: PhantomData,
        })
    }
}

///////////////////////////////////////////////////

struct PathVisitor;

impl<'a> Visitor<'a> for PathVisitor {
    type Value = &'a Path;

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.as_ref())
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        std::str::from_utf8(v)
            .map(AsRef::as_ref)
            .map_err(|_| Error::invalid_value("[borrowed bytes]", "bytes"))
    }
}

impl<'toy: 'a, 'a> Deserializable<'toy> for &'a Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_str(PathVisitor)
    }
}

///////////////////////////////////////////////////

struct PathBufVisitor;

impl<'a> Visitor<'a> for PathBufVisitor {
    type Value = PathBuf;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(From::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(From::from(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        std::str::from_utf8(v)
            .map(From::from)
            .map_err(|_| Error::invalid_value("[borrowed bytes]", "bytes"))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        String::from_utf8(v)
            .map(From::from)
            .map_err(|_| Error::invalid_value("[borrowed bytes]", "bytes"))
    }
}

impl<'toy> Deserializable<'toy> for PathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_string(PathBufVisitor)
    }
}

///////////////////////////////////////////////////

struct DurationVisitor;

impl<'a> Visitor<'a> for DurationVisitor {
    type Value = Duration;

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: DeserializeSeqOps<'a>,
    {
        let mut secs: Option<u64> = None;
        let mut nanos: Option<u32> = None;
        for filed_idx in 0..2usize {
            match filed_idx {
                0usize => {
                    secs = match seq.next::<u64>() {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    }
                }
                1usize => {
                    nanos = match seq.next::<u32>() {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    }
                }
                _ => match seq.next::<Discard>() {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                },
            }
        }
        let secs = match secs {
            Some(v) => v,
            None => Default::default(),
        };
        let nanos = match nanos {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(Duration::new(secs, nanos))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: DeserializeMapOps<'a>,
    {
        enum Field {
            Secs,
            Nanos,
            Unknown,
        }
        struct __FieldVisitor;
        impl<'toy> Visitor<'toy> for __FieldVisitor {
            type Value = Field;
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    0u32 => Ok(Field::Secs),
                    1u32 => Ok(Field::Nanos),
                    _ => Ok(Field::Unknown),
                }
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "secs" => Ok(Field::Secs),
                    "nanos" => Ok(Field::Nanos),
                    _ => Ok(Field::Unknown),
                }
            }
        }
        let mut secs: Option<u64> = None;
        let mut nanos: Option<u32> = None;
        while let Some(key) = map.next_identifier(__FieldVisitor)? {
            match key {
                Field::Secs => {
                    if Option::is_some(&secs) {
                        return Err(Error::duplicate_field("secs"));
                    }
                    secs = match map.next_value::<u64>() {
                        Ok(v) => Some(v),
                        Err(e) => return Err(e),
                    }
                }
                Field::Nanos => {
                    if Option::is_some(&nanos) {
                        return Err(Error::duplicate_field("nanos"));
                    }
                    nanos = match map.next_value::<u32>() {
                        Ok(v) => Some(v),
                        Err(e) => return Err(e),
                    }
                }
                _ => match map.next_value::<Discard>() {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                },
            };
        }
        let secs = match secs {
            Some(v) => v,
            None => Default::default(),
        };
        let nanos = match nanos {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(Duration::new(secs, nanos))
    }
}

impl<'toy> Deserializable<'toy> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_struct(DurationVisitor)
    }
}

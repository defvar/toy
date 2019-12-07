use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::{Deserializable, Deserializer, Error, Visitor};

impl<'toy, T> Deserializable<'toy> for PhantomData<T>
    where T: Deserializable<'toy>
{
    type Value = T::Value;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'toy>
    {
        T::deserialize(deserializer)
    }
}

///////////////////////////////////////////////////

impl<'toy, T> Deserializable<'toy> for Option<T> where T: Deserializable<'toy> {
    type Value = Option<T::Value>;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'toy>
    {
        struct OptionVisitor<T> {
            marker: PhantomData<T>,
        };

        impl<'a, T> Visitor<'a> for OptionVisitor<T> where T: Deserializable<'a> {
            type Value = Option<T::Value>;

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where D: Deserializer<'a>
            {
                T::deserialize(deserializer).map(Some)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
                Ok(None)
            }
        }

        deserializer.deserialize_option(OptionVisitor::<T> {
            marker: PhantomData,
        })
    }
}

///////////////////////////////////////////////////

impl<'toy: 'a, 'a> Deserializable<'toy> for &'a Path {
    type Value = &'a Path;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'toy>
    {
        struct PathVisitor;

        impl<'a> Visitor<'a> for PathVisitor {
            type Value = &'a Path;

            fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E> where E: Error {
                Ok(v.as_ref())
            }
        }

        deserializer.deserialize_str(PathVisitor)
    }
}

///////////////////////////////////////////////////

impl<'toy> Deserializable<'toy> for PathBuf {
    type Value = PathBuf;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'toy>
    {
        struct PathBufVisitor;

        impl<'a> Visitor<'a> for PathBufVisitor {
            type Value = PathBuf;

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
                Ok(From::from(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
                Ok(From::from(v))
            }
        }

        deserializer.deserialize_string(PathBufVisitor)
    }
}

use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::{Deserializable, Deserializer, Error, Visitor};
use crate::deser::discard::Discard;
use crate::deser::{DeserializeMapOps, DeserializeSeqOps};
use failure::_core::time::Duration;

impl<'toy, T> Deserializable<'toy> for PhantomData<T>
where
    T: Deserializable<'toy>,
{
    type Value = T::Value;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
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
    type Value = Box<T::Value>;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'toy>,
    {
        T::deserialize(deserializer).map(Box::new)
    }
}

///////////////////////////////////////////////////

impl<'toy, T> Deserializable<'toy> for Option<T>
where
    T: Deserializable<'toy>,
{
    type Value = Option<T::Value>;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct OptionVisitor<T> {
            marker: PhantomData<T>,
        };

        impl<'a, T> Visitor<'a> for OptionVisitor<T>
        where
            T: Deserializable<'a>,
        {
            type Value = Option<T::Value>;

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
    where
        D: Deserializer<'toy>,
    {
        struct PathVisitor;

        impl<'a> Visitor<'a> for PathVisitor {
            type Value = &'a Path;

            fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
            where
                E: Error,
            {
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
    where
        D: Deserializer<'toy>,
    {
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
        }

        deserializer.deserialize_string(PathBufVisitor)
    }
}

///////////////////////////////////////////////////

impl<'toy> Deserializable<'toy> for Duration {
    type Value = Duration;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, <D as Deserializer<'toy>>::Error>
    where
        D: Deserializer<'toy>,
    {
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
        deserializer.deserialize_struct(DurationVisitor)
    }
}

//! The URI of Node.
//!

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::Hash;

/// The URI of Node.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Uri {
    path: String,
}

impl From<&str> for Uri {
    fn from(v: &str) -> Self {
        Uri {
            path: v.to_string(),
        }
    }
}

impl From<String> for Uri {
    fn from(v: String) -> Self {
        Uri { path: v }
    }
}

impl From<&String> for Uri {
    fn from(v: &String) -> Self {
        Uri {
            path: v.to_string(),
        }
    }
}

impl From<&Uri> for Uri {
    fn from(v: &Uri) -> Self {
        v.clone()
    }
}

impl AsRef<Uri> for Uri {
    fn as_ref(&self) -> &Uri {
        self
    }
}

impl Debug for Uri {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.path.to_string())
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string())
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.path)
    }
}

impl<'toy> Deserialize<'toy> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct UriVisitor;

        impl<'a> Visitor<'a> for UriVisitor {
            type Value = Uri;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "error")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Uri::from(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Uri::from(v))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s =
                    std::str::from_utf8(v).map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(Uri::from(s))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s =
                    std::str::from_utf8(&v).map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(Uri::from(s))
            }
        }

        deserializer.deserialize_string(UriVisitor)
    }
}

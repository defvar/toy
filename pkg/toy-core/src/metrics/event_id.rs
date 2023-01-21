use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Formatter};
use std::str::FromStr;
use uuid::Uuid;

/// Event Record Identifier
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventId {
    id: Uuid,
}

impl EventId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from(uuid: Uuid) -> Self {
        Self { id: uuid }
    }

    pub fn parse_str<T: AsRef<str>>(uuid: T) -> Result<EventId, ()> {
        Uuid::parse_str(uuid.as_ref())
            .map(|id| EventId::from(id))
            .map_err(|_| ())
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl fmt::Debug for EventId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl Default for EventId {
    fn default() -> Self {
        EventId::from(Uuid::nil())
    }
}

impl FromStr for EventId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EventId::parse_str(s)
    }
}

impl<'toy> Deserialize<'toy> for EventId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct TaskIdVisitor;

        impl<'a> Visitor<'a> for TaskIdVisitor {
            type Value = EventId;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "error")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(v)
                    .map(EventId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(&v)
                    .map(EventId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(v)
                    .map(EventId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(&v)
                    .map(EventId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }
        }

        deserializer.deserialize_string(TaskIdVisitor)
    }
}

impl Serialize for EventId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.to_string().serialize(serializer)
    }
}

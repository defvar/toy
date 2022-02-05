use crate::selection::candidate::CandidateMap;
use crate::selection::field::Selection;
use serde::{Deserialize, Serialize};

/// Data format of the api.
/// Can be specified as an option when making a request.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "mp")]
    MessagePack,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

/// Traits that should be implemented by Key-Value object.
pub trait KVObject {
    fn key(&self) -> &str;
}

/// Traits that should be implemented by structs that are used as options for find-based api.
pub trait FindOptionLike {
    /// Returns the common option items.
    fn common(&self) -> &FindOption;
}

/// Traits that should be implemented by structs that are used as options for list-based api.
pub trait ListOptionLike {
    /// Returns the common option items.
    fn common(&self) -> &ListOption;

    /// Create `Selection`.
    fn selection(&self) -> Selection;
}

/// This trait is used to select data based on the specified conditions when calling api.
pub trait SelectionCandidate {
    /// Creates and returns the field information needed to make a selection.
    fn candidate_map(&self) -> CandidateMap;
}

/// Common option items for find-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindOption {
    format: Option<Format>,
    pretty: Option<bool>,
}

impl FindOption {
    pub fn new() -> Self {
        Self {
            format: None,
            pretty: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn pretty(&self) -> Option<bool> {
        self.pretty
    }

    pub fn with_pretty(self) -> Self {
        Self {
            pretty: Some(true),
            ..self
        }
    }
}

/// Common option items for list-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListOption {
    format: Option<Format>,
    pretty: Option<bool>,
}

impl ListOption {
    pub fn new() -> Self {
        Self {
            format: None,
            pretty: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn pretty(&self) -> Option<bool> {
        self.pretty
    }

    pub fn with_pretty(self) -> Self {
        Self {
            pretty: Some(true),
            ..self
        }
    }
}

/// Common option items for put-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutOption {
    format: Option<Format>,
    pretty: Option<bool>,
}

impl PutOption {
    pub fn new() -> Self {
        Self {
            format: None,
            pretty: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn pretty(&self) -> Option<bool> {
        self.pretty
    }

    pub fn with_pretty(self) -> Self {
        Self {
            pretty: Some(true),
            ..self
        }
    }
}

/// Common option items for delete-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteOption {
    format: Option<Format>,
    pretty: Option<bool>,
}

impl DeleteOption {
    pub fn new() -> Self {
        Self {
            format: None,
            pretty: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn pretty(&self) -> Option<bool> {
        self.pretty
    }

    pub fn with_pretty(self) -> Self {
        Self {
            pretty: Some(true),
            ..self
        }
    }
}

pub(crate) mod format {
    pub(crate) mod rfc3399 {
        use chrono::{DateTime, Utc};
        use core::fmt;
        use serde::{self, de, Deserializer, Serializer};

        pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&date.to_rfc3339())
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(TimestampVisitor)
        }

        pub(crate) struct TimestampVisitor;

        impl<'de> de::Visitor<'de> for TimestampVisitor {
            type Value = DateTime<Utc>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a timestamp rfc3399.")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match DateTime::parse_from_rfc3339(v) {
                    Ok(dt) => Ok(dt.with_timezone(&Utc)),
                    Err(e) => Err(E::custom(format!("Parse error {} for {}", e, v))),
                }
            }
        }
    }

    pub(crate) mod rfc3399_option {
        use super::rfc3399::TimestampVisitor;
        use chrono::{DateTime, Utc};
        use core::fmt;
        use serde::{self, de, Deserializer, Serializer};

        pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *date {
                Some(ref dt) => serializer.serialize_some(&dt.to_rfc3339()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_option(OptionTimestampVisitor)
        }

        struct OptionTimestampVisitor;

        impl<'de> de::Visitor<'de> for OptionTimestampVisitor {
            type Value = Option<DateTime<Utc>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a timestamp rfc3399.")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                d.deserialize_str(TimestampVisitor).map(Some)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }
    }
}

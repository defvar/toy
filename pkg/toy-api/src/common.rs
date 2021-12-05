use crate::selection::candidate::CandidateMap;
use crate::selection::field::Selection;
use serde::{Deserialize, Serialize};

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

pub trait FindOptionLike {
    fn common(&self) -> &FindOption;
}

pub trait ListOptionLike {
    fn common(&self) -> &ListOption;

    fn selection(&self) -> Selection;
}

pub trait SelectionCandidate {
    fn candidate_map(&self) -> CandidateMap;
}

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

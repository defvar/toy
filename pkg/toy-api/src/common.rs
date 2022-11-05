use crate::selection::candidate::Candidates;
use crate::selection::fields::Fields;
use crate::selection::selector::Selector;
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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Indent {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "pretty")]
    Pretty,
}

impl Default for Indent {
    fn default() -> Self {
        Indent::Pretty
    }
}

/// Traits that should be implemented by Key-Value object.
pub trait KVObject {
    fn key(&self) -> &str;
}

/// Traits that should be implemented by List object.
pub trait ListObject<T> {
    fn items(&self) -> &[T];

    fn count(&self) -> u32;
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
}

/// This trait is used to select data based on the specified conditions when calling api.
pub trait SelectionCandidate {
    /// Candidate field names.
    fn candidate_fields() -> &'static [&'static str];

    /// Creates and returns the field information needed to make a selection.
    fn candidates(&self) -> Candidates;
}

/// Common response for put-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonPutResponse {
    code: u16,
}

impl CommonPutResponse {
    pub fn with_code(code: u16) -> Self {
        Self { code }
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

/// Common option items for find-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindOption {
    format: Option<Format>,
    indent: Option<Indent>,
    #[serde(default)]
    fields: Fields,
}

impl FindOption {
    pub fn new() -> Self {
        Self {
            format: None,
            indent: None,
            fields: Fields::default(),
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn indent(&self) -> Option<Indent> {
        self.indent
    }

    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    pub fn with_pretty(self) -> Self {
        Self {
            indent: Some(Indent::Pretty),
            ..self
        }
    }
}

/// Common option items for list-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListOption {
    format: Option<Format>,
    indent: Option<Indent>,
    #[serde(default)]
    selector: Selector,
    #[serde(default)]
    fields: Fields,
}

impl ListOption {
    pub fn new() -> Self {
        Self {
            format: None,
            indent: None,
            selector: Selector::empty(),
            fields: Fields::default(),
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn indent(&self) -> Option<Indent> {
        self.indent
    }

    pub fn selection(&self) -> &Selector {
        &self.selector
    }

    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    pub fn with_pretty(self) -> Self {
        Self {
            indent: Some(Indent::Pretty),
            ..self
        }
    }
}

impl ListOptionLike for ListOption {
    fn common(&self) -> &ListOption {
        &self
    }
}

/// Common option items for put-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutOption {
    format: Option<Format>,
    indent: Option<Indent>,
}

impl PutOption {
    pub fn new() -> Self {
        Self {
            format: None,
            indent: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn indent(&self) -> Option<Indent> {
        self.indent
    }

    pub fn with_pretty(self) -> Self {
        Self {
            indent: Some(Indent::Pretty),
            ..self
        }
    }
}

/// Common option items for post-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostOption {
    format: Option<Format>,
    indent: Option<Indent>,
}

impl PostOption {
    pub fn new() -> Self {
        Self {
            format: None,
            indent: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn indent(&self) -> Option<Indent> {
        self.indent
    }

    pub fn with_format(self, format: Format) -> Self {
        Self {
            format: Some(format),
            ..self
        }
    }

    pub fn with_pretty(self) -> Self {
        Self {
            indent: Some(Indent::Pretty),
            ..self
        }
    }
}

/// Common option items for delete-based api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteOption {
    format: Option<Format>,
    indent: Option<Indent>,
}

impl DeleteOption {
    pub fn new() -> Self {
        Self {
            format: None,
            indent: None,
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn pretty(&self) -> Option<Indent> {
        self.indent
    }

    pub fn with_pretty(self) -> Self {
        Self {
            indent: Some(Indent::Pretty),
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
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
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

//! The URI of Node.
//!

use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display, Error, Formatter};

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

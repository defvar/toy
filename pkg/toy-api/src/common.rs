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

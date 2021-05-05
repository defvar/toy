use toy_pack::{Pack, Unpack};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Pack, Unpack)]
pub enum Format {
    #[toy(rename = "json")]
    Json,
    #[toy(rename = "yaml")]
    Yaml,
    #[toy(rename = "mp")]
    MessagePack,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct FindOption {
    format: Option<Format>,
}

impl FindOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ListOption {
    format: Option<Format>,
}

impl ListOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct PutOption {
    format: Option<Format>,
}

impl PutOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct DeleteOption {
    format: Option<Format>,
}

impl DeleteOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

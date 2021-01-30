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

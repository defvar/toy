use std::path::PathBuf;

use toy_pack_derive::*;

use super::{QuoteStyle, Terminator};

pub const fn default_capacity() -> usize {
    8 * (1 << 10)
}

pub fn char_to_u8(v: char) -> u8 {
    let mut dest = [0u8; 4];
    let _ = v.encode_utf8(&mut dest);
    dest[0]
}

#[derive(Debug, Clone, UnPack)]
pub struct FileConfig {
    pub(crate) source: Option<SourceConfig>,
    pub(crate) sink: Option<SinkConfig>,
}

impl FileConfig {
    pub fn new(source: Option<SourceConfig>, sink: Option<SinkConfig>) -> Self {
        Self { source, sink }
    }

    pub fn get_source_config(&self) -> Option<&SourceConfig> {
        self.source.as_ref()
    }

    pub fn get_sink_config(&self) -> Option<&SinkConfig> {
        self.sink.as_ref()
    }
}

#[derive(Debug, Clone, UnPack, Default)]
pub struct SourceConfig {
    pub(crate) kind: SourceType,
    pub(crate) path: Option<PathBuf>,
    pub(crate) option: SourceOption,
}

#[derive(Debug, Clone, UnPack)]
pub struct SourceOption {
    #[toy(default = ',')]
    pub(crate) delimiter: char,
    #[toy(default = '"')]
    pub(crate) quote: char,
    pub(crate) quoting: bool,
    pub(crate) terminator: Terminator,
    pub(crate) escape: Option<u8>,
    pub(crate) double_quote: bool,
    pub(crate) comment: Option<u8>,
    pub(crate) has_header: bool,
    pub(crate) flexible: bool,
    #[toy(default_expr = "default_capacity")]
    pub(crate) capacity: usize,
    pub(crate) columns: Option<Vec<Column>>,
}

impl Default for SourceOption {
    fn default() -> SourceOption {
        SourceOption {
            delimiter: ',',
            quote: '"',
            quoting: true,
            terminator: Terminator::default(),
            escape: None,
            double_quote: true,
            comment: None,
            capacity: default_capacity(),
            has_header: true,
            flexible: false,
            columns: None,
        }
    }
}

#[derive(Debug, Clone, UnPack, Default)]
pub struct Column {
    pub(crate) name: String,
}

#[derive(Debug, Clone, UnPack, Default)]
pub struct SinkConfig {
    pub(crate) kind: SinkType,
    pub(crate) path: Option<PathBuf>,
    pub(crate) option: SinkOption,
}

#[derive(Debug, Clone, UnPack)]
pub struct SinkOption {
    #[toy(default = ',')]
    pub(crate) delimiter: char,
    #[toy(default = '"')]
    pub(crate) quote: char,
    pub(crate) quote_style: QuoteStyle,
    pub(crate) terminator: Terminator,
    pub(crate) escape: u8,
    pub(crate) double_quote: bool,
    #[toy(default_expr = "default_capacity")]
    pub(crate) capacity: usize,
}

impl Default for SinkOption {
    fn default() -> SinkOption {
        SinkOption {
            capacity: default_capacity(),
            delimiter: ',',
            quote: '"',
            quote_style: QuoteStyle::default(),
            terminator: Terminator::default(),
            escape: b'\\',
            double_quote: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, UnPack)]
pub enum SourceType {
    File,
    Stdin,
}

impl Default for SourceType {
    fn default() -> Self {
        SourceType::Stdin
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, UnPack)]
pub enum SinkType {
    File,
    Stdout,
}

impl Default for SinkType {
    fn default() -> Self {
        SinkType::Stdout
    }
}

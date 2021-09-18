use super::QuoteStyle;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toy_pack::Schema;
use toy_text_parser::Terminator;

pub const fn default_capacity() -> usize {
    8 * (1 << 10)
}

const fn default_quote() -> char {
    '"'
}

const fn default_delimiter() -> char {
    ','
}

const fn default_has_headers() -> bool {
    true
}

pub fn char_to_u8(v: char) -> u8 {
    let mut dest = [0u8; 4];
    let _ = v.encode_utf8(&mut dest);
    dest[0]
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema)]
pub struct ReadConfig {
    pub(crate) kind: SourceType,
    pub(crate) path: Option<PathBuf>,
    #[serde(default)]
    pub(crate) option: ReadOption,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct ReadOption {
    #[serde(default = "default_delimiter")]
    pub(crate) delimiter: char,
    #[serde(default = "default_quote")]
    pub(crate) quote: char,
    pub(crate) quoting: bool,
    pub(crate) terminator: Terminator,
    pub(crate) escape: Option<u8>,
    pub(crate) double_quote: bool,
    pub(crate) comment: Option<u8>,
    #[serde(default = "default_has_headers")]
    pub(crate) has_headers: bool,
    pub(crate) flexible: bool,
    #[serde(default = "default_capacity")]
    pub(crate) capacity: usize,
}

impl Default for ReadOption {
    fn default() -> ReadOption {
        ReadOption {
            delimiter: ',',
            quote: '"',
            quoting: true,
            terminator: Terminator::default(),
            escape: None,
            double_quote: true,
            comment: None,
            capacity: default_capacity(),
            has_headers: true,
            flexible: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema)]
pub struct Column {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema)]
pub struct WriteConfig {
    pub(crate) kind: SinkType,
    pub(crate) path: Option<PathBuf>,
    #[serde(default)]
    pub(crate) option: WriteOption,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct WriteOption {
    #[serde(default = "default_has_headers")]
    pub(crate) has_headers: bool,
    #[serde(default = "default_delimiter")]
    pub(crate) delimiter: char,
    #[serde(default = "default_quote")]
    pub(crate) quote: char,
    pub(crate) quote_style: QuoteStyle,
    pub(crate) terminator: Terminator,
    pub(crate) escape: u8,
    pub(crate) double_quote: bool,
    #[serde(default = "default_capacity")]
    pub(crate) capacity: usize,
}

impl Default for WriteOption {
    fn default() -> WriteOption {
        WriteOption {
            has_headers: true,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Schema)]
pub enum SourceType {
    File,
    Stdin,
}

impl Default for SourceType {
    fn default() -> Self {
        SourceType::Stdin
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Schema)]
pub enum SinkType {
    File,
    Stdout,
}

impl Default for SinkType {
    fn default() -> Self {
        SinkType::Stdout
    }
}

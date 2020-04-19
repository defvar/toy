use std::path::PathBuf;

use toy_pack::UnPack;

use super::{QuoteStyle, Terminator};

pub const fn default_capacity() -> usize {
    8 * (1 << 10)
}

pub fn char_to_u8(v: char) -> u8 {
    let mut dest = [0u8; 4];
    let _ = v.encode_utf8(&mut dest);
    dest[0]
}

#[derive(Debug, Clone, UnPack, Default)]
pub struct FileReadConfig {
    pub(crate) kind: SourceType,
    pub(crate) path: Option<PathBuf>,
    pub(crate) option: ReadOption,
}

#[derive(Debug, Clone, UnPack)]
pub struct ReadOption {
    #[toy(default = ',')]
    pub(crate) delimiter: char,
    #[toy(default = '"')]
    pub(crate) quote: char,
    pub(crate) quoting: bool,
    pub(crate) terminator: Terminator,
    pub(crate) escape: Option<u8>,
    pub(crate) double_quote: bool,
    pub(crate) comment: Option<u8>,
    #[toy(default = true)]
    pub(crate) has_headers: bool,
    pub(crate) flexible: bool,
    #[toy(default_expr = "default_capacity")]
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

#[derive(Debug, Clone, UnPack, Default)]
pub struct Column {
    pub(crate) name: String,
}

#[derive(Debug, Clone, UnPack, Default)]
pub struct FileWriteConfig {
    pub(crate) kind: SinkType,
    pub(crate) path: Option<PathBuf>,
    pub(crate) option: WriteOption,
}

#[derive(Debug, Clone, UnPack)]
pub struct WriteOption {
    #[toy(default = true)]
    pub(crate) has_headers: bool,
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

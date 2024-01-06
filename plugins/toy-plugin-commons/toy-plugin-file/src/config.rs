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

const fn default_has_headers() -> bool {
    true
}

const fn default_escape() -> char {
    '\\'
}

pub fn char_to_u8(v: char) -> u8 {
    let mut dest = [0u8; 4];
    let _ = v.encode_utf8(&mut dest);
    dest[0]
}

pub fn char_to_u8_opt(v: Option<char>) -> Option<u8> {
    if v.is_some() {
        Some(char_to_u8(v.unwrap()))
    } else {
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema)]
pub struct ReadConfig {
    pub(crate) path: String,
    #[serde(default)]
    pub(crate) option: ReadOption,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct ReadOption {
    pub(crate) delimiter: Option<char>,
    #[serde(default = "default_quote")]
    pub(crate) quote: char,
    #[serde(default)]
    pub(crate) quoting: bool,
    #[serde(default)]
    pub(crate) terminator: Terminator,
    pub(crate) escape: Option<u8>,
    #[serde(default)]
    pub(crate) double_quote: bool,
    pub(crate) comment: Option<u8>,
    #[serde(default = "default_has_headers")]
    pub(crate) has_headers: bool,
    #[serde(default)]
    pub(crate) flexible: bool,
    #[serde(default = "default_capacity")]
    pub(crate) capacity: usize,
}

impl ReadConfig {
    pub fn new(path: String) -> ReadConfig {
        ReadConfig {
            path,
            option: ReadOption::default(),
        }
    }

    pub fn with(path: String, option: ReadOption) -> ReadConfig {
        ReadConfig { path, option }
    }
}

impl Default for ReadOption {
    fn default() -> ReadOption {
        ReadOption {
            delimiter: None,
            quote: '"',
            quoting: false,
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

impl ReadOption {
    pub fn common_csv() -> ReadOption {
        ReadOption {
            delimiter: Some(','),
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
pub struct WriteConfig {
    pub(crate) path: Option<PathBuf>,
    #[serde(default)]
    pub(crate) option: WriteOption,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct WriteOption {
    #[serde(default = "default_has_headers")]
    pub(crate) has_headers: bool,
    pub(crate) delimiter: Option<char>,
    #[serde(default = "default_quote")]
    pub(crate) quote: char,
    #[serde(default)]
    pub(crate) quote_style: QuoteStyle,
    #[serde(default)]
    pub(crate) terminator: Terminator,
    #[serde(default = "default_escape")]
    pub(crate) escape: char,
    #[serde(default)]
    pub(crate) double_quote: bool,
    #[serde(default = "default_capacity")]
    pub(crate) capacity: usize,
}

impl Default for WriteOption {
    fn default() -> WriteOption {
        WriteOption {
            has_headers: true,
            capacity: default_capacity(),
            delimiter: None,
            quote: '"',
            quote_style: QuoteStyle::default(),
            terminator: Terminator::default(),
            escape: '\\',
            double_quote: true,
        }
    }
}

impl WriteOption {
    pub fn common_csv() -> WriteOption {
        WriteOption {
            has_headers: true,
            capacity: default_capacity(),
            delimiter: Some(','),
            quote: '"',
            quote_style: QuoteStyle::default(),
            terminator: Terminator::default(),
            escape: '\\',
            double_quote: true,
        }
    }
}

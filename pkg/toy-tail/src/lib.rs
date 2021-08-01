#![feature(backtrace, type_alias_impl_trait)]

mod config;
mod error;
mod flagment;
mod handler;
pub mod handlers;
mod reader;
mod regex_parser;
mod tail;
mod watcher;

pub use self::config::{TailConfig, TailConfigBuilder};
pub use self::error::TailError;
pub use self::flagment::{Flagment, Flagments};
pub use self::handler::Handler;
pub use self::reader::LineReader;
pub use self::regex_parser::RegexParser;
pub use self::tail::TailContext;
pub use self::watcher::watch;

#[doc(hidden)]
pub use toy_text_parser::Line;

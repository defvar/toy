#![feature(backtrace, type_alias_impl_trait)]

#[doc(hidden)]
pub use toy_text_parser::Line;

pub use parsers::regex_parser::RegexParser;

pub use self::config::{TailConfig, TailConfigBuilder};
pub use self::error::TailError;
pub use self::flagment::{Flagment, Flagments};
pub use self::handler::{Handler, Handlers};
pub use self::reader::LineReader;
pub use self::tail::TailContext;
pub use self::watcher::watch;

mod config;
mod error;
mod flagment;
mod flush_timer;
mod handler;
pub mod handlers;
pub mod listeners;
pub mod parsers;
mod reader;
mod tail;
mod watcher;

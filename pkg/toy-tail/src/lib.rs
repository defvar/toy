#![feature(backtrace)]

mod error;
mod flagment;
mod handler;
mod reader;
mod regex_parser;
mod tail;
mod watcher;

pub use self::error::TailError;
pub use self::flagment::{Flagment, Flagments};
pub use self::handler::{Handler, PrintHandler};
pub use self::reader::LineReader;
pub use self::regex_parser::RegexParser;
pub use self::tail::TailContext;
pub use self::watcher::watch;

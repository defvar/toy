pub use self::reader::ByteReader;
pub use self::reader_builder::ReaderBuilder;
use self::states::{Action, State};
pub use self::states::ReadResult;

mod dfa;
mod reader;
mod reader_builder;
mod states;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    CRLF,
    Any(u8),
}

impl Terminator {
    pub fn is_crlf(&self) -> bool {
        match *self {
            Terminator::CRLF => true,
            Terminator::Any(_) => false,
        }
    }

    pub fn equals(&self, other: u8) -> bool {
        match *self {
            Terminator::CRLF => other == b'\r' || other == b'\n',
            Terminator::Any(b) => other == b,
        }
    }
}

impl Default for Terminator {
    fn default() -> Terminator {
        Terminator::CRLF
    }
}

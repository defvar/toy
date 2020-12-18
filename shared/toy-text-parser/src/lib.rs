mod dfa_parser;
mod terminator;

pub use self::terminator::Terminator;

pub mod dfa {
    pub use super::dfa_parser::ByteParser;
    pub use super::dfa_parser::ByteParserBuilder;
    pub use super::dfa_parser::ReadResult;
}

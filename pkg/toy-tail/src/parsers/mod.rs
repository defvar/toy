pub mod regex_parser;

use crate::Flagments;
use toy_text_parser::Line;

pub trait Parser {
    fn parse<'a>(&self, line: &'a Line) -> Flagments<'a>;
}

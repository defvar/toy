use super::dfa::Dfa;
use super::ByteParser;
use crate::Terminator;

#[derive(Clone)]
pub struct ByteParserBuilder {
    delimiter: Option<u8>,
    quote: u8,
    quoting: bool,
    terminator: Terminator,
    escape: Option<u8>,
    double_quote: bool,
    comment: Option<u8>,
}

impl ByteParserBuilder {
    pub fn delimiter(&mut self, c: u8) -> &mut Self {
        self.delimiter = Some(c);
        self
    }

    pub fn quote(&mut self, c: u8) -> &mut Self {
        self.quote = c;
        self
    }

    pub fn quoting(&mut self, yes: bool) -> &mut Self {
        self.quoting = yes;
        self
    }

    pub fn terminator(&mut self, t: Terminator) -> &mut Self {
        self.terminator = t;
        self
    }

    pub fn escape(&mut self, c: Option<u8>) -> &mut Self {
        self.escape = c;
        self
    }

    pub fn double_quote(&mut self, yes: bool) -> &mut Self {
        self.double_quote = yes;
        self
    }

    pub fn comment(&mut self, c: Option<u8>) -> &mut Self {
        self.comment = c;
        self
    }

    pub fn csv() -> ByteParserBuilder {
        Self {
            delimiter: Some(b','),
            quote: b'"',
            quoting: true,
            terminator: Terminator::default(),
            escape: None,
            double_quote: true,
            comment: None,
        }
    }

    pub fn build(&self) -> ByteParser {
        let mut d = Dfa::with_prop(
            self.delimiter,
            self.quote,
            self.quoting,
            self.terminator,
            self.escape,
            self.double_quote,
            self.comment,
        );
        let s = d.build();
        ByteParser::with_dfa(d, s)
    }
}

impl Default for ByteParserBuilder {
    fn default() -> Self {
        Self {
            delimiter: None,
            quote: b'"',
            quoting: false,
            terminator: Terminator::default(),
            escape: None,
            double_quote: true,
            comment: None,
        }
    }
}

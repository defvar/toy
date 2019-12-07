use super::ByteReader;
use super::dfa::Dfa;
use super::Terminator;

#[derive(Clone)]
pub struct ReaderBuilder {
    delimiter: u8,
    quote: u8,
    quoting: bool,
    terminator: Terminator,
    escape: Option<u8>,
    double_quote: bool,
    comment: Option<u8>,
}

impl ReaderBuilder {
    pub fn delimiter(&mut self, c: u8) -> &mut Self {
        self.delimiter = c;
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

    pub fn build(&self) -> ByteReader {
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
        ByteReader::with_dfa(d, s)
    }
}

impl Default for ReaderBuilder {
    fn default() -> Self {
        Self {
            delimiter: b',',
            quote: b'"',
            quoting: true,
            terminator: Terminator::default(),
            escape: None,
            double_quote: true,
            comment: None,
        }
    }
}

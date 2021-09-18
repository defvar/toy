use std::fs::File;
use std::io::{self, BufWriter, Error, ErrorKind};
use std::path::Path;

use super::config::{self, char_to_u8, SinkType, WriteConfig};
use super::file_writer::FileWriter;
use crate::QuoteStyle;
use toy_text_parser::Terminator;

#[derive(Clone)]
pub struct FileWriterBuilder {
    capacity: usize,
    has_headers: bool,
    delimiter: u8,
    quote: u8,
    quote_style: QuoteStyle,
    double_quote: bool,
    terminator: Terminator,
    escape: u8,
}

impl FileWriterBuilder {
    pub fn configure(config: &WriteConfig) -> Result<FileWriter<Box<dyn io::Write + Send>>, Error> {
        if config.kind == SinkType::File && config.path.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "type: file, path must be set of config.yml",
            ));
        }

        let b = FileWriterBuilder::default()
            .has_headers(config.option.has_headers)
            .delimiter(char_to_u8(config.option.delimiter))
            .quote(char_to_u8(config.option.quote))
            .quote_style(config.option.quote_style)
            .terminator(config.option.terminator)
            .escape(config.option.escape)
            .double_quote(config.option.double_quote)
            .capacity(config.option.capacity)
            .clone();

        Ok(b.from_writer(match config.kind {
            SinkType::Stdout => Box::new(io::stdout()),
            SinkType::File => Box::new(File::create(config.path.as_ref().unwrap().as_path())?),
        }))
    }

    pub fn capacity(&mut self, cap: usize) -> &mut Self {
        self.capacity = cap;
        self
    }

    pub fn has_headers(&mut self, yes: bool) -> &mut Self {
        self.has_headers = yes;
        self
    }

    pub fn delimiter(&mut self, c: u8) -> &mut Self {
        self.delimiter = c;
        self
    }

    pub fn quote(&mut self, c: u8) -> &mut Self {
        self.quote = c;
        self
    }

    pub fn quote_style(&mut self, style: QuoteStyle) -> &mut Self {
        self.quote_style = style;
        self
    }

    pub fn double_quote(&mut self, yes: bool) -> &mut Self {
        self.double_quote = yes;
        self
    }

    pub fn terminator(&mut self, t: Terminator) -> &mut Self {
        self.terminator = t;
        self
    }

    pub fn escape(&mut self, c: u8) -> &mut Self {
        self.escape = c;
        self
    }

    pub fn from_writer<W: io::Write>(&self, w: W) -> FileWriter<W> {
        FileWriter::new(
            BufWriter::with_capacity(self.capacity, w),
            self.has_headers,
            self.delimiter,
            self.quote,
            self.quote_style,
            self.terminator,
            self.escape,
            self.double_quote,
        )
    }

    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<FileWriter<File>, Error> {
        Ok(self.from_writer(File::create(path)?))
    }
}

impl Default for FileWriterBuilder {
    fn default() -> Self {
        Self {
            capacity: config::default_capacity(),
            has_headers: true,
            delimiter: b',',
            quote: b'"',
            quote_style: QuoteStyle::default(),
            terminator: Terminator::default(),
            escape: b'\\',
            double_quote: true,
        }
    }
}

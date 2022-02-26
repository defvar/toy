use super::config::{char_to_u8, default_capacity, ReadConfig};
use super::file_reader::{FileReader, FileReaderState};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::path::PathBuf;
use toy_text_parser::dfa::ByteParserBuilder;
use toy_text_parser::Terminator;

#[derive(Clone)]
pub struct FileReaderBuilder {
    parser_builder: ByteParserBuilder,
    capacity: usize,
    has_headers: bool,
}

impl FileReaderBuilder {
    pub fn configure(config: &ReadConfig) -> Result<FileReader, Error> {
        let mut paths: Vec<PathBuf> = match glob::glob(&config.path) {
            Ok(p) => p.map(|x| x.unwrap()).collect(),
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e.msg)),
        };

        if paths.len() == 0 {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("file not found. path: {}", &config.path),
            ));
        }

        paths.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let b = FileReaderBuilder::default()
            .delimiter(char_to_u8(config.option.delimiter))
            .quote(char_to_u8(config.option.quote))
            .quoting(config.option.quoting)
            .terminator(config.option.terminator)
            .escape(config.option.escape)
            .double_quote(config.option.double_quote)
            .comment(config.option.comment)
            .has_headers(config.option.has_headers)
            .capacity(config.option.capacity)
            .clone();

        Ok(b.from_file(File::open(paths.get(0).unwrap())?, paths))
    }

    pub fn capacity(&mut self, cap: usize) -> &mut Self {
        self.capacity = cap;
        self
    }

    pub fn has_headers(&mut self, yes: bool) -> &mut Self {
        self.has_headers = yes;
        self
    }

    pub fn from_file(&self, f: File, paths: Vec<PathBuf>) -> FileReader {
        FileReader::new(
            self.parser_builder.build(),
            BufReader::with_capacity(self.capacity, f),
            FileReaderState::new(self.has_headers, paths),
        )
    }

    // wrap parser builder //

    pub fn delimiter(&mut self, c: u8) -> &mut Self {
        self.parser_builder.delimiter(c);
        self
    }

    pub fn quote(&mut self, c: u8) -> &mut Self {
        self.parser_builder.quote(c);
        self
    }

    pub fn quoting(&mut self, yes: bool) -> &mut Self {
        self.parser_builder.quoting(yes);
        self
    }

    pub fn terminator(&mut self, t: Terminator) -> &mut Self {
        self.parser_builder.terminator(t);
        self
    }

    pub fn escape(&mut self, c: Option<u8>) -> &mut Self {
        self.parser_builder.escape(c);
        self
    }

    pub fn double_quote(&mut self, yes: bool) -> &mut Self {
        self.parser_builder.double_quote(yes);
        self
    }

    pub fn comment(&mut self, c: Option<u8>) -> &mut Self {
        self.parser_builder.comment(c);
        self
    }
}

impl Default for FileReaderBuilder {
    fn default() -> Self {
        Self {
            parser_builder: ByteParserBuilder::default(),
            capacity: default_capacity(),
            has_headers: true,
        }
    }
}

use super::config::{char_to_u8, default_capacity, FileReadConfig, SourceType};
use super::parse::ReaderBuilder;
use super::{FileReader, FileReaderState, Terminator};
use std::fs::File;
use std::io::{self, BufReader, Error, ErrorKind};
use std::path::Path;

#[derive(Clone)]
pub struct FileReaderBuilder {
    reader_builder: ReaderBuilder,
    capacity: usize,
    has_headers: bool,
    flexible: bool,
}

impl FileReaderBuilder {
    pub fn configure(config: &FileReadConfig) -> Result<FileReader<Box<dyn io::Read>>, Error> {
        if config.kind == SourceType::File && config.path.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "type: file, path must be set of config.yml",
            ));
        }

        let b = FileReaderBuilder::default()
            .delimiter(char_to_u8(config.option.delimiter))
            .quote(char_to_u8(config.option.quote))
            .quoting(config.option.quoting)
            .terminator(config.option.terminator)
            .escape(config.option.escape)
            .double_quote(config.option.double_quote)
            .comment(config.option.comment)
            .has_headers(config.option.has_headers)
            .flexible(config.option.flexible)
            .capacity(config.option.capacity)
            .clone();

        Ok(b.from_reader(match config.kind {
            SourceType::Stdin => Box::new(io::stdin()),
            SourceType::File => Box::new(File::open(config.path.as_ref().unwrap().as_path())?),
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

    pub fn flexible(&mut self, yes: bool) -> &mut Self {
        self.flexible = yes;
        self
    }

    pub fn from_reader<R: io::Read>(&self, r: R) -> FileReader<R> {
        FileReader::new(
            self.reader_builder.build(),
            BufReader::with_capacity(self.capacity, r),
            FileReaderState::new(self.has_headers, self.flexible),
        )
    }

    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<FileReader<File>, Error> {
        Ok(self.from_reader(File::open(path)?))
    }

    // wrap reader builder //

    pub fn delimiter(&mut self, c: u8) -> &mut Self {
        self.reader_builder.delimiter(c);
        self
    }

    pub fn quote(&mut self, c: u8) -> &mut Self {
        self.reader_builder.quote(c);
        self
    }

    pub fn quoting(&mut self, yes: bool) -> &mut Self {
        self.reader_builder.quoting(yes);
        self
    }

    pub fn terminator(&mut self, t: Terminator) -> &mut Self {
        self.reader_builder.terminator(t.to_parse());
        self
    }

    pub fn escape(&mut self, c: Option<u8>) -> &mut Self {
        self.reader_builder.escape(c);
        self
    }

    pub fn double_quote(&mut self, yes: bool) -> &mut Self {
        self.reader_builder.double_quote(yes);
        self
    }

    pub fn comment(&mut self, c: Option<u8>) -> &mut Self {
        self.reader_builder.comment(c);
        self
    }
}

impl Default for FileReaderBuilder {
    fn default() -> Self {
        Self {
            reader_builder: ReaderBuilder::default(),
            capacity: default_capacity(),
            has_headers: true,
            flexible: false,
        }
    }
}

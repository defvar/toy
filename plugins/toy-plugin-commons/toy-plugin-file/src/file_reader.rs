use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

use toy_text_parser::dfa::{ByteParser, ParseResult};
use toy_text_parser::Line;

#[derive(Debug)]
pub struct FileReader {
    reader: ByteParser,
    src: BufReader<File>,
    state: FileReaderState,
}

#[derive(Debug)]
pub struct FileReaderState {
    has_headers: bool,
    headers: Option<Line>,
    current_path_index: usize,
    paths: Vec<PathBuf>,
    eof: bool,
    has_read: bool,
}

impl FileReaderState {
    pub fn new(has_headers: bool, paths: Vec<PathBuf>) -> FileReaderState {
        FileReaderState {
            has_headers,
            headers: None,
            current_path_index: 0,
            paths,
            eof: false,
            has_read: false,
        }
    }

    pub fn prepare_next_file(&mut self) -> Option<&PathBuf> {
        self.eof = false;
        self.has_read = false;
        self.current_path_index += 1;
        self.headers = None;
        self.paths.get(self.current_path_index)
    }
}

impl FileReader {
    /// Create a new Source given a built `ByteReader` and a source underlying IO reader.
    ///
    pub fn new(reader: ByteParser, src: BufReader<File>, state: FileReaderState) -> FileReader {
        FileReader { reader, src, state }
    }

    /// Returns Iterator all Row.
    ///
    pub fn rows(&mut self) -> RowIterator {
        RowIterator::new(self)
    }

    /// Returns IntoIterator all Row.
    ///
    pub fn into_rows(self) -> RowIntoIterator {
        RowIntoIterator::new(self)
    }

    /// Returns true if reader configured to first row as a header.
    ///
    pub fn has_headers(&self) -> bool {
        self.state.has_headers
    }

    /// Returns a reference to the first row.
    ///
    /// If has been read yet, then this will force parsing of the first row.
    ///
    pub fn headers(&mut self) -> Result<&Line, Error> {
        if self.state.headers.is_none() {
            let mut line = Line::new();
            self.read_core(&mut line)?;
            self.state.headers = Some(line.clone());
        }
        Ok(&self.state.headers.as_ref().unwrap())
    }

    /// Read a single row into the given `Row`.
    /// Returns false when no more records could be read.
    ///
    pub fn read(&mut self, line: &mut Line) -> Result<bool, Error> {
        let r = self.read_current_path(line)?;

        // next file
        if !r && (self.state.current_path_index + 1) < self.state.paths.len() {
            self.reader.reset();
            let cp = self.src.capacity();
            let next_path = self.state.prepare_next_file().unwrap();
            tracing::info!("read next file. path: {}", next_path.display());
            let next = File::open(next_path)?;
            self.src = BufReader::with_capacity(cp, next);
            return self.read_current_path(line);
        }
        Ok(r)
    }

    fn read_current_path(&mut self, line: &mut Line) -> Result<bool, Error> {
        let r = self.read_core(line)?;

        // skip header, once more read.
        if self.state.has_headers && self.state.headers.is_none() {
            self.state.headers = Some(line.clone());
            return self.read_core(line);
        }

        Ok(r)
    }

    #[inline]
    fn read_core(&mut self, line: &mut Line) -> Result<bool, Error> {
        line.clear();
        self.state.has_read = true;

        if self.state.eof {
            return Ok(false);
        }

        let (mut out_pos, mut column) = (0, 0);
        loop {
            let (state, in_size, out_size, col) = {
                let input = self.src.fill_buf()?;
                let (buf, edges) = line.parts();
                self.reader
                    .read_record(input, &mut buf[out_pos..], &mut edges[column..])
            };

            self.src.consume(in_size);

            column += col;
            out_pos += out_size;

            match state {
                ParseResult::OutputFull => {
                    line.expand_force_columns();
                    continue;
                }
                ParseResult::OutputEdgeFull => {
                    line.expand_force_edges();
                    continue;
                }
                ParseResult::InputEmpty => continue,
                ParseResult::End => {
                    self.state.eof = true;
                    return Ok(false);
                }
                ParseResult::Record => {
                    line.set_len(column);
                    return Ok(true);
                }
            }
        }
    }
}

pub struct RowIterator<'a> {
    src: &'a mut FileReader,
    line: Line,
}

impl<'a> RowIterator<'a> {
    fn new(src: &'a mut FileReader) -> RowIterator<'a> {
        RowIterator {
            src,
            line: Line::new(),
        }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Result<Line, Error>;

    fn next(&mut self) -> Option<Result<Line, Error>> {
        match self.src.read(&mut self.line) {
            Ok(true) => Some(Ok(self.line.clone())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct RowIntoIterator {
    src: FileReader,
    line: Line,
}

impl RowIntoIterator {
    fn new(src: FileReader) -> RowIntoIterator {
        RowIntoIterator {
            src,
            line: Line::new(),
        }
    }
}

impl Iterator for RowIntoIterator {
    type Item = Result<Line, Error>;

    fn next(&mut self) -> Option<Result<Line, Error>> {
        match self.src.read(&mut self.line) {
            Ok(true) => Some(Ok(self.line.clone())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

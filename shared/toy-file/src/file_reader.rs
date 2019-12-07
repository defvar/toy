use std::io::{BufRead, BufReader, Error, Read};

use super::parse::{ByteReader, ReadResult};
use super::Row;

#[derive(Debug)]
pub struct FileReader<R> {
    reader: Box<ByteReader>,
    src: BufReader<R>,
    state: FileReaderState,
}

#[derive(Debug)]
pub struct FileReaderState {
    has_headers: bool,
    headers: Option<Row>,
    flexible: bool,

    eof: bool,
    has_read: bool,
}

impl FileReaderState {
    pub fn new(has_headers: bool, flexible: bool) -> FileReaderState {
        FileReaderState {
            has_headers,
            headers: None,
            flexible,

            eof: false,
            has_read: false,
        }
    }
}

impl<R: Read> FileReader<R> {

    /// Create a new Source given a built `ByteReader` and a source underlying IO reader.
    ///
    pub fn new(reader: Box<ByteReader>, src: BufReader<R>, state: FileReaderState) -> FileReader<R> {
        FileReader { reader, src, state }
    }

    /// Returns Iterator all Row.
    ///
    pub fn rows(&mut self) -> RowIterator<R> {
        RowIterator::new(self)
    }

    /// Returns IntoIterator all Row.
    ///
    pub fn into_rows(self) -> RowIntoIterator<R> {
        RowIntoIterator::new(self)
    }

    /// Returns a reference to the first row.
    ///
    /// If has been read yet, then this will force parsing of the first row.
    /// `has_header`
    ///
    pub fn headers(&mut self) -> Result<&Row, Error> {
        if self.state.headers.is_none() {
            let mut row = Row::new();
            self.read_core(&mut row)?;
            self.state.headers = Some(row.clone());
        }
        Ok(&self.state.headers.as_ref().unwrap())
    }

    /// Read a single row into the given `Row`.
    /// Returns false when no more records could be read.
    ///
    pub fn read(&mut self, row: &mut Row) -> Result<bool, Error> {
        let r = self.read_core(row)?;

        // skip header, once more read.
        if self.state.has_headers && self.state.headers.is_none() {
            self.state.headers = Some(row.clone());
            return self.read_core(row);
        }

        Ok(r)
    }

    #[inline]
    fn read_core(&mut self, row: &mut Row) -> Result<bool, Error> {
        row.clear();
        self.state.has_read = true;

        if self.state.eof {
            return Ok(false);
        }

        let (mut out_pos, mut column) = (0, 0);
        loop {
            let (state, in_size, out_size, col) = {
                let input = self.src.fill_buf()?;
                let (buf, edges) = row.parts();
                self.reader.read_record(input, &mut buf[out_pos..], &mut edges[column..])
            };

            self.src.consume(in_size);

            column += col;
            out_pos += out_size;

            match state {
                ReadResult::OutputFull => {
                    row.expand_force_columns();
                    continue;
                }
                ReadResult::OutputEdgeFull => {
                    row.expand_force_edges();
                    continue;
                }
                ReadResult::InputEmpty => continue,
                ReadResult::End => {
                    self.state.eof = true;
                    return Ok(false);
                }
                ReadResult::Record => {
                    row.set_len(column);
                    return Ok(true);
                }
            }
        }
    }
}

pub struct RowIterator<'a, R: 'a> {
    src: &'a mut FileReader<R>,
    row: Row,
}

impl<'a, R: Read> RowIterator<'a, R> {
    fn new(src: &'a mut FileReader<R>) -> RowIterator<'a, R> {
        RowIterator { src, row: Row::new() }
    }
}

impl<'a, R: Read> Iterator for RowIterator<'a, R> {
    type Item = Result<Row, Error>;

    fn next(&mut self) -> Option<Result<Row, Error>> {
        match self.src.read(&mut self.row) {
            Ok(true) => Some(Ok(self.row.clone())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct RowIntoIterator<R> {
    src: FileReader<R>,
    row: Row,
}

impl<R: Read> RowIntoIterator<R> {
    fn new(src: FileReader<R>) -> RowIntoIterator<R> {
        RowIntoIterator {
            src,
            row: Row::new(),
        }
    }
}

impl<R: Read> Iterator for RowIntoIterator<R> {
    type Item = Result<Row, Error>;

    fn next(&mut self) -> Option<Result<Row, Error>> {
        match self.src.read(&mut self.row) {
            Ok(true) => Some(Ok(self.row.clone())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

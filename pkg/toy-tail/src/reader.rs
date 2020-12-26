use std::io::{BufRead, BufReader, Error, Read};
use toy_text_parser::dfa::{ByteParser, ReadResult};
use toy_text_parser::Line;

#[derive(Debug)]
pub struct LineReader<R> {
    parser: ByteParser,
    src: BufReader<R>,
}

impl<R: Read> LineReader<R> {
    /// Create a new Source given a built `ByteParser` and a source underlying IO reader.
    ///
    pub fn new(parser: ByteParser, src: BufReader<R>) -> LineReader<R> {
        LineReader { parser, src }
    }

    /// Set next reader.
    /// Reuse ByteParser.
    pub fn next(&mut self, src: BufReader<R>) {
        self.src = src;
        self.parser.reset();
    }

    /// Read a one line into the given `Line`.
    /// Returns false when no more records could be read.
    ///
    pub fn read(&mut self, line: &mut Line) -> Result<bool, Error> {
        let r = self.read_core(line)?;
        Ok(r)
    }

    #[inline]
    fn read_core(&mut self, line: &mut Line) -> Result<bool, Error> {
        let (mut out_pos, mut column) = (line.len_bytes(), line.len());
        loop {
            let (state, in_size, out_size, col) = {
                let input = self.src.fill_buf()?;
                let (buf, edges) = line.parts();
                self.parser
                    .read_record(input, &mut buf[out_pos..], &mut edges[column..])
            };

            self.src.consume(in_size);

            column += col;
            out_pos += out_size;

            match state {
                ReadResult::OutputFull => {
                    line.expand_force_columns();
                    continue;
                }
                ReadResult::OutputEdgeFull => {
                    line.expand_force_edges();
                    continue;
                }
                ReadResult::InputEmpty => continue,
                ReadResult::End => {
                    return Ok(false);
                }
                ReadResult::Record => {
                    line.set_len(column);
                    return Ok(true);
                }
            }
        }
    }
}

use std::fmt;
use std::io::{BufWriter, Error, Write};

use super::{QuoteStyle, Row, Terminator};

pub struct FileWriter<W: Write> {
    raw: BufWriter<W>,
    delimiter: u8,
    requires_quotes: [bool; 256],
    quote: u8,
    quote_style: QuoteStyle,
    terminator: Terminator,
    escape: u8,
    double_quote: bool,
    state: FileWriterState,
}

#[derive(Debug)]
pub struct FileWriterState {
    wrote_bytes: u64,
    wrote_row: u64,
}

impl<W: Write> FileWriter<W> {
    pub fn new(raw: BufWriter<W>,
               delimiter: u8,
               quote: u8,
               quote_style: QuoteStyle,
               terminator: Terminator,
               escape: u8,
               double_quote: bool) -> FileWriter<W> {
        let mut requires_quotes = [false; 256];
        requires_quotes[delimiter as usize] = true;
        requires_quotes[quote as usize] = true;
        if !double_quote {
            requires_quotes[escape as usize] = true;
        }
        match terminator {
            Terminator::CRLF | Terminator::Any(b'\r') | Terminator::Any(b'\n') => {
                requires_quotes[b'\r' as usize] = true;
                requires_quotes[b'\n' as usize] = true;
            }
            Terminator::Any(b) => requires_quotes[b as usize] = true,
        }

        FileWriter {
            raw,
            delimiter,
            requires_quotes,
            quote,
            quote_style,
            terminator,
            escape,
            double_quote,
            state: FileWriterState {
                wrote_bytes: 0,
                wrote_row: 0,
            },
        }
    }

    pub fn write_iter<I, T>(&mut self, iter: I) -> Result<(), Error>
        where I: IntoIterator<Item=T>, T: AsRef<[u8]>
    {
        let mut need_delimiter = false;

        for col in iter.into_iter() {
            self.write_column(col.as_ref(), need_delimiter)?;
            need_delimiter = true;
        }

        self.write_terminator()?;
        self.state.wrote_row += 1;
        Ok(())
    }

    pub fn write(&mut self, row: &Row) -> Result<(), Error> {
        self.write_iter(row)
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.raw.flush()
    }

    pub fn get_wrote_bytes(&self) -> u64 {
        self.state.wrote_bytes
    }

    pub fn get_wrote_row(&self) -> u64 {
        self.state.wrote_row
    }

    fn write_column(&mut self, col: &[u8], need_delimiter: bool) -> Result<(), Error> {
        let mut s = 0u64;

        if need_delimiter {
            self.raw.write(&[self.delimiter])?;
            s += 1u64;
        }

        if self.should_quote(col) {
            self.raw.write(&[self.quote])?;
            s += self.write_column_should_escape(col)? as u64;
            self.raw.write(&[self.quote])?;
            s += 2u64;
        } else {
            s += self.write_column_should_escape(col)? as u64;
        }

        self.state.wrote_bytes += s;
        Ok(())
    }

    fn write_column_should_escape(&mut self, col: &[u8]) -> Result<usize, Error> {
        if col.is_empty() {
            return Ok(0);
        }

        let mut wrote = 0;
        let mut wrote_size = 0;

        for (pos, b) in col.iter().enumerate() {
            if *b == self.quote {
                wrote_size += self.raw.write(&col[wrote..pos])?;
                wrote = pos + 1; //skip quote.
                if self.double_quote {
                    wrote_size += self.raw.write(&[self.quote, self.quote])?;
                } else {
                    wrote_size += self.raw.write(&[self.escape, self.quote])?;
                }
            }
        }

        if wrote < col.len() - 1 {
            wrote_size += self.raw.write(&col[wrote..])?;
        }

        Ok(wrote_size)
    }

    #[inline]
    fn write_terminator(&mut self) -> Result<(), Error> {
        let r = match self.terminator {
            Terminator::Any(b) => self.raw.write(&[b]),
            Terminator::CRLF => self.raw.write(&[b'\r', b'\n']),
        };

        match r {
            Ok(s) => {
                self.state.wrote_bytes += s as u64;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    #[inline]
    fn should_quote(&self, input: &[u8]) -> bool {
        match self.quote_style {
            QuoteStyle::Always => true,
            QuoteStyle::Never => false,
            QuoteStyle::Necessary => {
                input.iter().any(|&b| {
                    self.requires_quotes[b as usize]
                })
            }
        }
    }
}

impl<W: Write> fmt::Debug for FileWriter<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Sink")
            .field("state", &self.state)
            .field("delimiter", &self.delimiter)
            .field("quote", &self.quote)
            .field("quote_style", &self.quote_style)
            .field("double quote", &self.double_quote)
            .field("terminator", &self.terminator)
            .field("escape", &self.escape)
            .finish()
    }
}

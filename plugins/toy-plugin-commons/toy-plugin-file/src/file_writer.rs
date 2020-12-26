use std::fmt;
use std::io::{BufWriter, Error, IntoInnerError, Write};

use super::QuoteStyle;
use toy_core::data::Value;
use toy_text_parser::{Line, Terminator};

macro_rules! itoa_write {
    ($tp: ident, $fun_name: ident) => {
        fn $fun_name(&mut self, v: $tp, need_delimiter: bool) -> Result<(), Error> {
            let mut buf = itoa::Buffer::new();
            let b = buf.format(v).as_bytes();
            self.write_column(b, need_delimiter)
        }
    };
}

macro_rules! ryu_write {
    ($tp: ident, $fun_name: ident) => {
        fn $fun_name(&mut self, v: $tp, need_delimiter: bool) -> Result<(), Error> {
            let mut buf = ryu::Buffer::new();
            let b = buf.format(v).as_bytes();
            self.write_column(b, need_delimiter)
        }
    };
}

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
    has_headers: bool,
    wrote_bytes: u64,
    wrote_row: u64,
}

impl<W: Write> FileWriter<W> {
    pub fn new(
        raw: BufWriter<W>,
        has_headers: bool,
        delimiter: u8,
        quote: u8,
        quote_style: QuoteStyle,
        terminator: Terminator,
        escape: u8,
        double_quote: bool,
    ) -> FileWriter<W> {
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
                has_headers,
                wrote_bytes: 0,
                wrote_row: 0,
            },
        }
    }

    pub fn write_iter<I, T>(&mut self, iter: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<[u8]>,
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

    pub fn write(&mut self, line: &Line) -> Result<(), Error> {
        self.write_iter(line)
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.raw.flush()
    }

    pub fn into_inner(self) -> Result<W, IntoInnerError<BufWriter<W>>> {
        self.raw.into_inner()
    }

    pub fn get_wrote_bytes(&self) -> u64 {
        self.state.wrote_bytes
    }

    pub fn get_wrote_row(&self) -> u64 {
        self.state.wrote_row
    }

    itoa_write!(u8, write_value_u8);
    itoa_write!(u16, write_value_u16);
    itoa_write!(u32, write_value_u32);
    itoa_write!(u64, write_value_u64);
    itoa_write!(i8, write_value_i8);
    itoa_write!(i16, write_value_i16);
    itoa_write!(i32, write_value_i32);
    itoa_write!(i64, write_value_i64);
    ryu_write!(f32, write_value_f32);
    ryu_write!(f64, write_value_f64);

    pub fn write_value(&mut self, value: &Value) -> Result<(), Error> {
        if self.state.has_headers && self.state.wrote_row == 0 {
            let mut buf = Vec::new();
            self.write_value_headers(value, false, &mut buf)?;
            self.write_terminator()?;
            self.state.wrote_row += 1;
        }
        self.write_value_0(value, false)?;

        self.write_terminator()?;
        self.state.wrote_row += 1;
        Ok(())
    }

    fn write_value_0(&mut self, value: &Value, need_delimiter: bool) -> Result<(), Error> {
        match value {
            Value::Bool(v) => {
                self.write_column(if *v { b"true" } else { b"false" }, need_delimiter)
            }
            Value::U8(v) => self.write_value_u8(*v, need_delimiter),
            Value::U16(v) => self.write_value_u16(*v, need_delimiter),
            Value::U32(v) => self.write_value_u32(*v, need_delimiter),
            Value::U64(v) => self.write_value_u64(*v, need_delimiter),
            Value::I8(v) => self.write_value_i8(*v, need_delimiter),
            Value::I16(v) => self.write_value_i16(*v, need_delimiter),
            Value::I32(v) => self.write_value_i32(*v, need_delimiter),
            Value::I64(v) => self.write_value_i64(*v, need_delimiter),
            Value::F32(v) => self.write_value_f32(*v, need_delimiter),
            Value::F64(v) => self.write_value_f64(*v, need_delimiter),
            Value::String(v) => self.write_column(v.as_bytes(), need_delimiter),
            Value::Bytes(v) => self.write_column(v.as_slice(), need_delimiter),
            Value::Map(map) => self.write_inner_values(map.values(), need_delimiter),
            Value::Seq(seq) => self.write_inner_values(seq.iter(), need_delimiter),
            Value::Some(v) => self.write_value_0(v, need_delimiter),
            Value::TimeStamp(_) => Ok(()),
            Value::None | Value::Unit => Ok(()),
        }
    }

    fn write_inner_values<'a, I>(&mut self, values: I, need_delimiter: bool) -> Result<(), Error>
    where
        I: Iterator<Item = &'a Value>,
    {
        let mut need_delimiter = need_delimiter;
        for col in values {
            self.write_value_0(col, need_delimiter)?;
            need_delimiter = true;
        }
        Ok(())
    }

    fn write_value_headers(
        &mut self,
        value: &Value,
        need_delimiter: bool,
        parent: &Vec<u8>,
    ) -> Result<(), Error> {
        match value {
            Value::Map(map) => {
                let mut need_delimiter = need_delimiter;
                let mut text = Vec::<u8>::new();
                for (k, v) in map {
                    if parent.len() > 0 {
                        text.extend_from_slice(parent.as_slice());
                        text.push(b'.');
                    }
                    text.extend_from_slice(k.as_bytes());
                    self.write_value_headers(v, need_delimiter, &text)?;
                    need_delimiter = true;
                    text.clear();
                }
                Ok(())
            }
            Value::Seq(seq) => {
                let mut need_delimiter = need_delimiter;
                let mut text = Vec::<u8>::new();
                for (idx, v) in seq.iter().enumerate() {
                    if parent.len() > 0 {
                        text.extend_from_slice(parent.as_slice());
                        text.push(b'.');
                    }
                    let mut buf = itoa::Buffer::new();
                    let b = buf.format(idx).as_bytes();
                    text.extend_from_slice(b);
                    self.write_value_headers(v, need_delimiter, &text)?;
                    need_delimiter = true;
                    text.clear();
                }
                Ok(())
            }
            Value::Some(v) => self.write_value_headers(v, need_delimiter, parent),
            _ => {
                let text = if parent.len() == 0 {
                    "unknown".as_bytes()
                } else {
                    parent.as_slice()
                };
                self.write_column(text, need_delimiter)
            }
        }
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

        if wrote < col.len() {
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
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn should_quote(&self, input: &[u8]) -> bool {
        match self.quote_style {
            QuoteStyle::Always => true,
            QuoteStyle::Never => false,
            QuoteStyle::Necessary => input.iter().any(|&b| self.requires_quotes[b as usize]),
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

use std::io;

use super::Result;
use crate::decode::{Reference, Token};
use crate::DecodeError;

/// reader position.
/// use error detail. (e.g. error at line:4, column:12)
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Input source for Decoder.
///
pub trait Reader<'toy> {
    fn remaining(&self) -> usize;

    fn discard(&mut self, len: usize) -> Result<()>;

    fn position(&self) -> Position;

    fn index(&self) -> usize;

    fn peek(&mut self) -> Result<Option<u8>>;

    /// Peek byte.
    ///
    /// Until valid charactor.
    #[inline]
    fn peek_until(&mut self) -> Result<Option<u8>> {
        loop {
            match self.peek()? {
                Some(b'\r') | Some(b'\n') | Some(b'\t') | Some(b' ') => {
                    self.consume();
                }
                other => return Ok(other),
            };
        }
    }

    #[inline]
    fn peek_token(&mut self) -> Result<Option<Token>> {
        let fb = self.peek_until()?;
        Ok(match fb {
            Some(b'{') => Some(Token::BeginObject),
            Some(b'}') => Some(Token::EndObject),
            Some(b'[') => Some(Token::BeginArray),
            Some(b']') => Some(Token::EndArray),
            Some(b't') => Some(Token::True),
            Some(b'f') => Some(Token::False),
            Some(b'n') => Some(Token::Null),
            Some(b',') => Some(Token::Comma),
            Some(b':') => Some(Token::Colon),
            Some(b'-') => Some(Token::Number),
            Some(b'0'..=b'9') => Some(Token::Number),
            Some(b'"') => Some(Token::String),
            Some(other) => Some(Token::Unexpected(other)),
            None => None,
        })
    }

    fn next(&mut self) -> Result<Option<u8>>;

    fn consume(&mut self);

    #[inline]
    fn next_or_eof(&mut self) -> Result<u8> {
        match self.next() {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err(DecodeError::eof_while_parsing_value()),
            Err(e) => Err(e),
        }
    }

    fn parse_escape(&mut self, scratch: &mut Vec<u8>) -> Result<()> {
        let b = self.next()?;
        match b {
            Some(b'"') => scratch.push(b'"'),
            Some(b'\\') => scratch.push(b'\\'),
            Some(b'/') => scratch.push(b'/'),
            Some(b'b') => scratch.push(b'\x08'),
            Some(b'f') => scratch.push(b'\x0c'),
            Some(b'n') => scratch.push(b'\n'),
            Some(b'r') => scratch.push(b'\r'),
            Some(b't') => scratch.push(b'\t'),
            Some(b'u') => {
                fn encode_surrogate(scratch: &mut Vec<u8>, n: u16) {
                    scratch.extend_from_slice(&[
                        (n >> 12 & 0b0000_1111) as u8 | 0b1110_0000,
                        (n >> 6 & 0b0011_1111) as u8 | 0b1000_0000,
                        (n & 0b0011_1111) as u8 | 0b1000_0000,
                    ]);
                }

                let c = match self.decode_hex_escape()? {
                    n @ 0xDC00..=0xDFFF => {
                        encode_surrogate(scratch, n);
                        return Ok(());
                    }
                    n1 @ 0xD800..=0xDBFF => {
                        if self.peek()? == Some(b'\\') {
                            self.consume();
                        } else {
                            encode_surrogate(scratch, n1);
                            return Ok(());
                        }

                        if self.peek()? == Some(b'u') {
                            self.consume();
                        } else {
                            encode_surrogate(scratch, n1);
                            return self.parse_escape(scratch);
                        }

                        let n2 = self.decode_hex_escape()?;
                        if n2 < 0xDC00 || n2 > 0xDFFF {
                            return Err(DecodeError::error("LoneLeadingSurrogateInHexEscape"));
                        }

                        let n = (((n1 - 0xD800) as u32) << 10 | (n2 - 0xDC00) as u32) + 0x1_0000;
                        match char::from_u32(n) {
                            Some(c) => c,
                            None => return Err(DecodeError::error("InvalidUnicodeCodePoint")),
                        }
                    }
                    n => char::from_u32(n as u32).unwrap(),
                };
                scratch.extend_from_slice(c.encode_utf8(&mut [0_u8; 4]).as_bytes());
            }
            Some(_) => {
                return Err(DecodeError::error("InvalidEscape"));
            }
            None => return Err(DecodeError::eof_while_parsing_value()),
        }
        Ok(())
    }

    /// Assumes the previous byte was a hex escape sequnce ('\u') in a string.
    /// Parses next hexadecimal sequence.
    #[doc(hidden)]
    fn decode_hex_escape(&mut self) -> Result<u16>;

    fn decode_str_bytes<'a, F, T>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
        result: F,
    ) -> Result<Reference<'toy, 'a, T>>
    where
        T: ?Sized + 'a,
        F: for<'f> FnOnce(&'a Self, &'f [u8]) -> Result<&'f T>;
}

/// Input source that reads from a slice of bytes.
///
pub struct SliceReader<'a> {
    raw: &'a [u8],
    position: usize,
    cache: Option<u8>,
}

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            raw: slice,
            position: 0,
            cache: None,
        }
    }

    #[inline]
    fn advance(&mut self, count: usize) {
        let pos = self.position + count;

        assert!(pos <= self.raw.len());

        self.position = pos;
    }

    #[inline]
    fn get_byte(&mut self) -> Result<Option<u8>> {
        if self.remaining() > 0 {
            let p = self.position;
            self.advance(1);
            Ok(Some(self.raw[p]))
        } else {
            Ok(None)
        }
    }
}

impl<'toy> Reader<'toy> for SliceReader<'toy> {
    #[inline]
    fn remaining(&self) -> usize {
        let len = self.raw.len();
        let pos = self.position;

        if pos >= len {
            0
        } else {
            len - pos
        }
    }

    #[inline]
    fn discard(&mut self, len: usize) -> Result<()> {
        self.advance(len);
        Ok(())
    }

    #[inline]
    fn position(&self) -> Position {
        position_of_index(&self.raw, self.position)
    }

    fn index(&self) -> usize {
        self.position
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(v) => Ok(Some(v)),
            None => {
                let v = self.get_byte()?;
                self.cache = v;
                Ok(v)
            }
        }
    }

    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(b) => {
                self.cache = None;
                Ok(Some(b))
            }
            None => self.get_byte(),
        }
    }

    fn consume(&mut self) {
        assert!(self.cache.is_some());
        self.cache = None;
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        if self.position + 4 > self.raw.len() {
            self.position = self.raw.len();
            return Err(DecodeError::eof_while_parsing_value());
        }

        let mut n = 0;
        for _ in 0..4 {
            let ch = decode_hex_val(self.raw[self.position]);
            self.position += 1;
            match ch {
                None => return Err(DecodeError::error("InvalidEscape")),
                Some(val) => {
                    n = (n << 4) + val;
                }
            }
        }
        Ok(n)
    }

    fn decode_str_bytes<'a, F, T>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
        result: F,
    ) -> Result<Reference<'toy, 'a, T>>
    where
        T: ?Sized + 'a,
        F: for<'f> FnOnce(&'a Self, &'f [u8]) -> Result<&'f T>,
    {
        match self.peek_token()? {
            Some(Token::String) => {
                self.consume();
                let mut start = self.index();
                loop {
                    let b = self.next_or_eof()?;
                    if !ESCAPE[b as usize] {
                        continue;
                    }
                    match b {
                        b'"' => {
                            return if buf.is_empty() {
                                let borrowed = &self.raw[start..self.index() - 1];
                                result(self, borrowed).map(Reference::Borrowed)
                            } else {
                                buf.extend_from_slice(&self.raw[start..self.index() - 1]);
                                result(self, buf).map(Reference::Copied)
                            }
                        }
                        b'\\' => {
                            buf.extend_from_slice(&self.raw[start..self.index() - 1]);
                            self.parse_escape(buf)?;
                            start = self.index();
                        }
                        _ => return Err(DecodeError::error("ControlCharacterWhileParsingString")),
                    }
                }
            }
            Some(other) => Err(DecodeError::invalid_token(other, "String")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }
}

/// Input source that reads from a std::io.
///
pub struct IoReader<R> {
    raw: R,
    line: usize,
    column: usize,
    index: usize,
    cache: Option<u8>,
}

impl<R: io::Read> IoReader<R> {
    pub fn new(raw: R) -> Self {
        Self {
            raw,
            line: 1,
            column: 0,
            index: 0,
            cache: None,
        }
    }

    #[inline]
    fn get_byte(&mut self) -> Result<Option<u8>> {
        let mut r = [0u8; 1];
        let size = self.raw.read(&mut r)?;
        if size > 0 {
            self.index = self.index.checked_add(1).unwrap_or(usize::MAX);
            if r[0] == b'\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            Ok(Some(r[0]))
        } else {
            Ok(None)
        }
    }
}

impl<'toy, R> Reader<'toy> for IoReader<R>
where
    R: io::Read,
{
    #[inline]
    fn remaining(&self) -> usize {
        usize::MAX
    }

    #[inline]
    fn discard(&mut self, len: usize) -> Result<()> {
        for _ in 0..len {
            let mut r = [0u8; 1];
            match self.raw.read(&mut r) {
                Ok(0) => break,
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    #[inline]
    fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn index(&self) -> usize {
        self.index
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(v) => Ok(Some(v)),
            None => {
                let v = self.get_byte()?;
                self.cache = v;
                Ok(v)
            }
        }
    }

    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(b) => {
                self.cache = None;
                Ok(Some(b))
            }
            None => self.get_byte(),
        }
    }

    fn consume(&mut self) {
        assert!(self.cache.is_some());
        self.cache = None;
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        let mut n = 0;
        for _ in 0..4 {
            match decode_hex_val(self.next_or_eof()?) {
                None => return Err(DecodeError::error("InvalidEscape")),
                Some(val) => {
                    n = (n << 4) + val;
                }
            }
        }
        Ok(n)
    }

    fn decode_str_bytes<'a, F, T>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
        result: F,
    ) -> Result<Reference<'toy, 'a, T>>
    where
        T: ?Sized + 'a,
        F: for<'f> FnOnce(&'a Self, &'f [u8]) -> Result<&'f T>,
    {
        match self.peek_token()? {
            Some(Token::String) => {
                self.consume();
                loop {
                    let b = self.next_or_eof()?;
                    if !ESCAPE[b as usize] {
                        buf.push(b);
                        continue;
                    }
                    match b {
                        b'"' => {
                            return result(self, buf).map(Reference::Copied);
                        }
                        b'\\' => self.parse_escape(buf)?,
                        _ => return Err(DecodeError::error("ControlCharacterWhileParsingString")),
                    }
                }
            }
            Some(other) => Err(DecodeError::invalid_token(other, "String")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }
}

fn position_of_index(buf: &[u8], i: usize) -> Position {
    let mut position = Position { line: 1, column: 0 };
    for ch in &buf[..i] {
        match *ch {
            b'\n' => {
                position.line += 1;
                position.column = 0;
            }
            _ => {
                position.column += 1;
            }
        }
    }
    position
}

static ESCAPE: [bool; 256] = {
    const CT: bool = true; // control character \x00..=\x1F
    const QU: bool = true; // quote \x22
    const BS: bool = true; // backslash \x5C
                           //noinspection RsConstNaming
    const __: bool = false; // allow unescaped
    [
        //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
        CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, // 0
        CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, // 1
        __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
        __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
    ]
};

static HEX: [u8; 256] = {
    const __: u8 = 255; // not a hex digit
    [
        //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 0
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 1
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
        00, 01, 02, 03, 04, 05, 06, 07, 08, 09, __, __, __, __, __, __, // 3
        __, 10, 11, 12, 13, 14, 15, __, __, __, __, __, __, __, __, __, // 4
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 5
        __, 10, 11, 12, 13, 14, 15, __, __, __, __, __, __, __, __, __, // 6
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
    ]
};

fn decode_hex_val(val: u8) -> Option<u16> {
    let n = HEX[val as usize] as u16;
    if n == 255 {
        None
    } else {
        Some(n)
    }
}

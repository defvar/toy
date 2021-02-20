use super::reader::Reader;
use super::{DecodeError, Reference, Result, Token};
use crate::decode::{ParseNumber, Position};
use lexical::FromLexical;

pub struct Decoder<B> {
    reader: B,
    cache: Option<u8>,

    parse_number_buffer: Vec<u8>,
}

impl<'toy, B> Decoder<B>
where
    B: Reader<'toy>,
{
    pub fn new(reader: B) -> Decoder<B> {
        Decoder {
            reader,
            cache: None,
            parse_number_buffer: Vec::new(),
        }
    }

    #[inline]
    pub fn position(&self) -> Position {
        self.reader.position()
    }

    pub fn decode_bool(&mut self) -> Result<bool> {
        match self.peek_token()? {
            Some(Token::True) => {
                self.consume();
                self.parse_ident(b"rue")?;
                Ok(true)
            }
            Some(Token::False) => {
                self.consume();
                self.parse_ident(b"alse")?;
                Ok(false)
            }
            Some(other) => Err(DecodeError::invalid_token(other, "True or False")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    pub fn decode_number(&mut self) -> Result<ParseNumber> {
        match self.peek_token()? {
            Some(Token::Number) => {
                self.parse_number_buffer.clear();
                let mut is_negative = false;
                let mut is_float = false;

                if self.peek()? == Some(b'-') {
                    is_negative = true;
                    let b = self.next_or_eof()?;
                    self.parse_number_buffer.push(b);
                }
                loop {
                    match self.peek_or_null()? {
                        b'0'..=b'9' => {
                            let b = self.next_or_eof()?;
                            self.parse_number_buffer.push(b);
                        }
                        b'.' | b'e' | b'E' | b'+' | b'-' => {
                            is_float = true;
                            let b = self.next_or_eof()?;
                            self.parse_number_buffer.push(b);
                        }
                        _ => break,
                    }
                }

                if is_float {
                    self.to_parse_number::<f64>(&self.parse_number_buffer)
                        .map(ParseNumber::F64)
                } else if is_negative {
                    self.to_parse_number::<i64>(&self.parse_number_buffer)
                        .map(ParseNumber::I64)
                } else {
                    self.to_parse_number::<u64>(&self.parse_number_buffer)
                        .map(ParseNumber::U64)
                }
            }
            Some(other) => Err(DecodeError::invalid_token(other, "Number")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    fn to_parse_number<T: FromLexical>(&self, chars: &Vec<u8>) -> Result<T> {
        lexical::parse::<T, _>(chars)
            .map_err(|_| DecodeError::invalid_number(self.reader.position()))
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

    pub fn decode_str<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> Result<Reference<'toy, 'a, str>> {
        self.decode_str_bytes(buf, |_, bytes| {
            std::str::from_utf8(bytes).map_err(|e| e.into())
        })
    }

    pub fn decode_str_raw<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> Result<Reference<'toy, 'a, [u8]>> {
        self.decode_str_bytes(buf, |_, bytes| Ok(bytes))
    }

    pub fn parse_escape(&mut self, scratch: &mut Vec<u8>) -> Result<()> {
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
            Some(_) => {
                return Err(DecodeError::error("InvalidEscape"));
            }
            None => return Err(DecodeError::eof_while_parsing_value()),
        }
        Ok(())
    }

    #[inline]
    pub fn next(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(b) => {
                self.cache = None;
                Ok(Some(b))
            }
            None => self.reader.get_byte(),
        }
    }

    #[inline]
    pub fn next_or_eof(&mut self) -> Result<u8> {
        match self.next() {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err(DecodeError::eof_while_parsing_value()),
            Err(e) => Err(e),
        }
    }

    pub fn end_seq(&mut self) -> Result<()> {
        match self.peek_token()? {
            Some(Token::EndArray) => {
                self.consume();
                Ok(())
            }
            Some(Token::Comma) => {
                self.consume();
                match self.peek_token()? {
                    Some(Token::EndArray) => Err(DecodeError::trailing_comma(self.position())),
                    Some(_) => Err(DecodeError::error("TrailingCharacters")),
                    None => Err(DecodeError::eof_while_parsing_value()),
                }
            }
            _ => Err(DecodeError::error("TrailingCharacters")),
        }
    }

    pub fn end_map(&mut self) -> Result<()> {
        match self.peek_token()? {
            Some(Token::EndObject) => {
                let _ = self.next()?;
                Ok(())
            }
            Some(Token::Comma) => Err(DecodeError::trailing_comma(self.position())),
            Some(_) => Err(DecodeError::error("TrailingCharacters")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    /// peek byte as a `Token`.
    #[inline]
    pub fn peek_token(&mut self) -> Result<Option<Token>> {
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

    /// Peek byte.
    ///
    #[inline]
    pub fn peek(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(v) => Ok(Some(v)),
            None => {
                let v = self.reader.get_byte()?;
                self.cache = v;
                Ok(v)
            }
        }
    }

    /// Peek byte.
    ///
    /// If eof then null charactor.
    ///
    #[inline]
    pub fn peek_or_null(&mut self) -> Result<u8> {
        Ok(self.peek()?.unwrap_or(b'\x00'))
    }

    /// Peek byte.
    ///
    /// Until valid charactor.
    ///
    #[inline]
    pub fn peek_until(&mut self) -> Result<Option<u8>> {
        loop {
            match self.peek()? {
                Some(b'\r') | Some(b'\n') | Some(b'\t') | Some(b' ') => {
                    self.consume();
                }
                other => return Ok(other),
            };
        }
    }

    /// Consume the peeked byte.
    ///
    /// Must be after peeked.
    ///
    #[inline]
    pub fn consume(&mut self) {
        assert!(self.cache.is_some());
        self.cache = None;
    }

    pub fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
        for expected in ident {
            match self.next()? {
                Some(next) => {
                    if next != *expected {
                        return Err(DecodeError::error("expected some ident"));
                    }
                }
                None => return Err(DecodeError::eof_while_parsing_value()),
            }
        }
        Ok(())
    }
}

static ESCAPE: [bool; 256] = {
    const CT: bool = true; // control character \x00..=\x1F
    const QU: bool = true; // quote \x22
    const BS: bool = true; // backslash \x5C
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

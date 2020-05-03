use super::reader::Reader;
use super::{DecodeError, Reference, Result, Token};
use crate::decode::ParseNumber;

pub struct Decoder<B> {
    reader: B,
    cache: Option<u8>,
}

impl<'toy, B> Decoder<B>
where
    B: Reader<'toy>,
{
    pub fn new(reader: B) -> Decoder<B> {
        Decoder {
            reader,
            cache: None,
        }
    }

    pub fn decode_bool(&mut self) -> Result<bool> {
        match self.peek_token()? {
            Some(Token::True) => {
                let _ = self.next()?;
                self.parse_ident(b"rue")?;
                Ok(true)
            }
            Some(Token::False) => {
                let _ = self.next()?;
                self.parse_ident(b"alse")?;
                Ok(false)
            }
            Some(other) => Err(DecodeError::from(other)),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    pub fn decode_number(&mut self) -> Result<ParseNumber> {
        match self.peek_token()? {
            Some(Token::Number) => {
                let mut negative = false;
                let mut r = 0u64;
                let mut r2 = 0u64;

                if self.peek()? == Some(b'-') {
                    negative = true;
                    let _ = self.next()?;
                }

                loop {
                    match self.peek_or_null()? {
                        b'0'..=b'9' => {
                            let b = self.next_or_eof()?;
                            r = r * 10 + ((b - b'0') as u64);
                        }
                        b'.' => {
                            let _ = self.next()?;
                            loop {
                                match self.peek_or_null()? {
                                    b'0'..=b'9' => {
                                        let b = self.next_or_eof()?;
                                        r2 = r2 * 10 + ((b - b'0') as u64);
                                    }
                                    _ => break,
                                }
                            }
                            break;
                        }
                        _ => break,
                    }
                }
                if r2 == 0 {
                    if negative {
                        Ok(ParseNumber::I64(-(r as i64)))
                    } else {
                        Ok(ParseNumber::U64(r))
                    }
                } else {
                    match format!("{}.{}", r, r2).parse::<f64>() {
                        Ok(v) => {
                            if negative {
                                Ok(ParseNumber::F64(-v))
                            } else {
                                Ok(ParseNumber::F64(v))
                            }
                        }
                        Err(e) => {
                            return Err(DecodeError::error(format!("parse float error: {:?}", e)))
                        }
                    }
                }
            }
            Some(other) => Err(DecodeError::from(other)),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

    pub fn decode_str<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> Result<Reference<'toy, 'a, str>> {
        match self.peek_token()? {
            Some(Token::String) => {
                let _ = self.next()?;
                loop {
                    let b = self.next_or_eof()?;
                    if !ESCAPE[b as usize] {
                        buf.push(b);
                        continue;
                    }
                    match b {
                        b'"' => {
                            return Ok(unsafe {
                                Reference::Copied(std::str::from_utf8_unchecked(buf.as_slice()))
                            });
                        }
                        b'\\' => self.parse_escape(buf)?,
                        _ => return Err(DecodeError::error("ControlCharacterWhileParsingString")),
                    }
                }
            }
            Some(other) => Err(DecodeError::from(other)),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
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
    fn get_byte(&mut self) -> Result<Option<u8>> {
        self.reader.get_byte()
    }

    #[inline]
    pub fn next(&mut self) -> Result<Option<u8>> {
        match self.cache {
            Some(b) => {
                self.cache = None;
                Ok(Some(b))
            }
            None => self.get_byte(),
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
        match self.peek_until()? {
            Some(b']') => {
                let _ = self.next()?;
                Ok(())
            }
            Some(b',') => {
                let _ = self.next()?;
                match self.peek_until()? {
                    Some(b']') => Err(DecodeError::error("TrailingComma")),
                    Some(_) => Err(DecodeError::error("TrailingCharacters")),
                    None => Err(DecodeError::eof_while_parsing_value()),
                }
            }
            _ => Err(DecodeError::error("TrailingCharacters")),
        }
    }

    pub fn end_map(&mut self) -> Result<()> {
        match self.peek_until()? {
            Some(b'}') => {
                let _ = self.next()?;
                Ok(())
            }
            Some(b',') => Err(DecodeError::error("TrailingComma")),
            Some(_) => Err(DecodeError::error("TrailingCharacters")),
            None => Err(DecodeError::eof_while_parsing_value()),
        }
    }

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
            Some(b',') => Some(Token::ValueSeparator),
            Some(b':') => Some(Token::NameSeparator),
            Some(b'-') => Some(Token::Number),
            Some(b'0'..=b'9') => Some(Token::Number),
            Some(b'\"') => Some(Token::String),
            Some(other) => Some(Token::Unexpected(other)),
            None => None,
        })
    }

    #[inline]
    pub fn peek(&mut self) -> Result<Option<u8>> {
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
    pub fn peek_or_null(&mut self) -> Result<u8> {
        Ok(self.peek()?.unwrap_or(b'\x00'))
    }

    #[inline]
    pub fn peek_until(&mut self) -> Result<Option<u8>> {
        loop {
            match self.peek()? {
                Some(b'\r') | Some(b'\n') | Some(b'\t') | Some(b' ') => {
                    let _ = self.next()?;
                }
                other => return Ok(other),
            };
        }
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

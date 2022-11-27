use super::reader::Reader;
use super::{DecodeError, Reference, Result, Token};
use crate::decode::{ParseNumber, Position};
use lexical::FromLexical;

pub struct Decoder<B> {
    reader: B,
    parse_number_buffer: Vec<u8>,
}

impl<'toy, B> Decoder<B>
where
    B: Reader<'toy>,
{
    pub fn new(reader: B) -> Decoder<B> {
        Decoder {
            reader,
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

    pub fn decode_str<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> Result<Reference<'toy, 'a, str>> {
        self.reader.decode_str_bytes(buf, |_, bytes| {
            //std::str::from_utf8(bytes).map_err(|e| e.into())
            Ok(unsafe { std::str::from_utf8_unchecked(bytes) })
        })
    }

    pub fn decode_str_raw<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> Result<Reference<'toy, 'a, [u8]>> {
        self.reader.decode_str_bytes(buf, |_, bytes| Ok(bytes))
    }

    #[inline]
    pub fn next(&mut self) -> Result<Option<u8>> {
        self.reader.next()
    }

    #[inline]
    pub fn next_or_eof(&mut self) -> Result<u8> {
        self.reader.next_or_eof()
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
        self.reader.peek_token()
    }

    /// Peek byte.
    #[inline]
    pub fn peek(&mut self) -> Result<Option<u8>> {
        self.reader.peek()
    }

    /// Peek byte.
    ///
    /// If eof then null charactor.
    #[inline]
    pub fn peek_or_null(&mut self) -> Result<u8> {
        Ok(self.reader.peek()?.unwrap_or(b'\x00'))
    }

    /// Peek byte.
    ///
    /// Until valid charactor.
    #[inline]
    pub fn peek_until(&mut self) -> Result<Option<u8>> {
        self.reader.peek_until()
    }

    /// Consume the peeked byte.
    ///
    /// Must be after peeked.
    #[inline]
    pub fn consume(&mut self) {
        self.reader.consume()
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

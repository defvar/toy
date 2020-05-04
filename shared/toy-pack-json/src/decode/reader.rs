use std::io;

use super::Result;

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

    fn get_byte(&mut self) -> Result<Option<u8>>;

    fn discard(&mut self, len: usize) -> Result<()>;

    fn position(&self) -> Position;
}

/// Input source that reads from a slice of bytes.
///
pub struct SliceReader<'a> {
    raw: &'a [u8],
    position: usize,
}

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            raw: slice,
            position: 0,
        }
    }

    #[inline]
    fn advance(&mut self, count: usize) {
        let pos = self.position + count;

        assert!(pos <= self.raw.len());

        self.position = pos;
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
    fn get_byte(&mut self) -> Result<Option<u8>> {
        if self.remaining() > 0 {
            let p = self.position;
            self.advance(1);
            Ok(Some(self.raw[p]))
        } else {
            Ok(None)
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
}

/// Input source that reads from a std::io.
///
pub struct IoReader<R> {
    raw: R,
    line: usize,
    column: usize,
}

impl<R: io::Read> IoReader<R> {
    pub fn new(raw: R) -> Self {
        Self {
            raw,
            line: 1,
            column: 0,
        }
    }
}

impl<'toy, R> Reader<'toy> for IoReader<R>
where
    R: io::Read,
{
    #[inline]
    fn remaining(&self) -> usize {
        usize::max_value()
    }

    #[inline]
    fn get_byte(&mut self) -> Result<Option<u8>> {
        let mut r = [0u8; 1];
        let size = self.raw.read(&mut r)?;
        if size > 0 {
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

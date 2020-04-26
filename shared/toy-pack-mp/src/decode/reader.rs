use std::io;

use super::{Reference, Result};

/// Input source for Decoder.
///
pub trait Reader<'toy> {
    fn remaining(&self) -> usize;

    fn get_byte(&mut self) -> Result<u8>;

    fn get_bytes<'a>(&'a mut self, len: usize) -> Result<Reference<'toy, 'a, [u8]>>;

    fn discard(&mut self, len: usize) -> Result<()>;
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
    fn get_byte(&mut self) -> Result<u8> {
        let p = self.position;
        self.advance(1);
        Ok(self.raw[p])
    }

    #[inline]
    fn get_bytes<'a>(&'a mut self, len: usize) -> Result<Reference<'toy, 'a, [u8]>> {
        let p = self.position;
        self.advance(len);
        Ok(Reference::Borrowed(&self.raw[p..p + len]))
    }

    #[inline]
    fn discard(&mut self, len: usize) -> Result<()> {
        self.advance(len);
        Ok(())
    }
}

/// Input source that reads from a std::io.
///
pub struct IoReader<R> {
    raw: R,
    buffer: Vec<u8>,
}

impl<R: io::Read> IoReader<R> {
    pub fn new(raw: R) -> Self {
        Self {
            raw,
            buffer: vec![0u8; 128],
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
    fn get_byte(&mut self) -> Result<u8> {
        let mut r = [0u8; 1];
        self.raw.read(&mut r)?;
        Ok(r[0])
    }

    #[inline]
    fn get_bytes<'a>(&'a mut self, len: usize) -> Result<Reference<'toy, 'a, [u8]>> {
        if self.buffer.capacity() < len {
            self.buffer.resize(len, 0u8);
        }
        self.raw.read_exact(&mut self.buffer[..len])?;
        Ok(Reference::Copied(&self.buffer[..len]))
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
}

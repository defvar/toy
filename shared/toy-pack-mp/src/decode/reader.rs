use std::io;
use std::mem;
use crate::DecodeError;

use super::{Reference, Result};

/// Input source for Decoder.
///
pub trait Reader<'toy> {
    fn remaining(&self) -> usize;

    fn get_byte(&mut self) -> Result<u8>;

    fn get_bytes<'a>(&'a mut self, len: usize) -> Result<Reference<'toy, 'a, [u8]>>;

    fn get_raw_u16(&mut self) -> Result<u16>;

    fn get_raw_u32(&mut self) -> Result<u32>;

    fn get_raw_u64(&mut self) -> Result<u64>;

    fn get_raw_i8(&mut self) -> Result<i8>;

    fn get_raw_i16(&mut self) -> Result<i16>;

    fn get_raw_i32(&mut self) -> Result<i32>;

    fn get_raw_i64(&mut self) -> Result<i64>;

    fn get_raw_f32(&mut self) -> Result<f32>;

    fn get_raw_f64(&mut self) -> Result<f64>;

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

macro_rules! get_raw_slice {
    ($t: ident, $variant: ident, $slice_fun: ident) => {
        #[inline]
        fn $variant(&mut self) -> Result<$t> {
            let p = self.position;
            const LEN: usize = mem::size_of::<$t>();
            self.advance(LEN);
            $slice_fun(&self.raw[p..p + LEN])
        }
    };
}

macro_rules! get_raw_io {
    ($t: ident, $variant: ident, $slice_fun: ident) => {
        #[inline]
        fn $variant(&mut self) -> Result<$t> {
            const LEN: usize = mem::size_of::<$t>();
            if self.buffer.capacity() < LEN {
                self.buffer.resize(LEN, 0u8);
            }
            self.raw.read_exact(&mut self.buffer[..LEN])?;
            $slice_fun(&self.buffer[..LEN])
        }
    };
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

    get_raw_slice!(u16, get_raw_u16, read_slice_u16);
    get_raw_slice!(u32, get_raw_u32, read_slice_u32);
    get_raw_slice!(u64, get_raw_u64, read_slice_u64);

    get_raw_slice!(i8, get_raw_i8, read_slice_i8);
    get_raw_slice!(i16, get_raw_i16, read_slice_i16);
    get_raw_slice!(i32, get_raw_i32, read_slice_i32);
    get_raw_slice!(i64, get_raw_i64, read_slice_i64);

    get_raw_slice!(f32, get_raw_f32, read_slice_f32);
    get_raw_slice!(f64, get_raw_f64, read_slice_f64);

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

    get_raw_io!(u16, get_raw_u16, read_slice_u16);
    get_raw_io!(u32, get_raw_u32, read_slice_u32);
    get_raw_io!(u64, get_raw_u64, read_slice_u64);

    get_raw_io!(i8, get_raw_i8, read_slice_i8);
    get_raw_io!(i16, get_raw_i16, read_slice_i16);
    get_raw_io!(i32, get_raw_i32, read_slice_i32);
    get_raw_io!(i64, get_raw_i64, read_slice_i64);

    get_raw_io!(f32, get_raw_f32, read_slice_f32);
    get_raw_io!(f64, get_raw_f64, read_slice_f64);

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

macro_rules! read_slice {
    ($t: ident, $variant: ident) => {
        fn $variant(slice: &[u8]) -> Result<$t> {
            slice.try_into().map(|x| $t::from_be_bytes(x)).map_err(|e| DecodeError::error(e))
        }
    };
}

read_slice!(u16, read_slice_u16);
read_slice!(u32, read_slice_u32);
read_slice!(u64, read_slice_u64);

read_slice!(i8, read_slice_i8);
read_slice!(i16, read_slice_i16);
read_slice!(i32, read_slice_i32);
read_slice!(i64, read_slice_i64);

read_slice!(f32, read_slice_f32);
read_slice!(f64, read_slice_f64);

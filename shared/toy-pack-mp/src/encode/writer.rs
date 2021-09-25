use std::io;
use std::{mem, slice};

use super::Result;

pub trait Writer {
    fn remaining(&self) -> usize;

    fn put<T: Sized>(&mut self, v: T);

    fn put_slice<T: Sized>(&mut self, v: &[T]);

    fn put_byte(&mut self, v: u8) -> Result<()>;
}

pub struct IoWriter<W> {
    raw: W,
}

impl<W: io::Write> IoWriter<W> {
    pub fn new(raw: W) -> Self {
        Self { raw }
    }
}

impl<W: io::Write> Writer for IoWriter<W> {
    #[inline]
    fn remaining(&self) -> usize {
        usize::MAX
    }

    #[inline]
    fn put<T: Sized>(&mut self, v: T) {
        self.put_slice(&[v])
    }

    #[inline]
    fn put_slice<T: Sized>(&mut self, v: &[T]) {
        let s = mem::size_of::<T>() * (*v).len();
        let bytes = unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, s) };
        self.raw.write_all(bytes).unwrap()
    }

    #[inline]
    fn put_byte(&mut self, v: u8) -> Result<()> {
        self.raw.write(&[v]).map(|_| Ok(()))?
    }
}

pub struct VecWriter<'a> {
    raw: &'a mut Vec<u8>,
}

impl<'a> VecWriter<'a> {
    pub fn new(raw: &'a mut Vec<u8>) -> Self {
        Self { raw }
    }
}

impl<'a> Writer for VecWriter<'a> {
    #[inline]
    fn remaining(&self) -> usize {
        usize::MAX
    }

    #[inline]
    fn put<T: Sized>(&mut self, v: T) {
        self.put_slice(&[v])
    }

    #[inline]
    fn put_slice<T: Sized>(&mut self, v: &[T]) {
        let s = mem::size_of::<T>() * (*v).len();
        let bytes = unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, s) };
        self.raw.extend_from_slice(bytes);
    }

    #[inline]
    fn put_byte(&mut self, v: u8) -> Result<()> {
        self.raw.push(v);
        Ok(())
    }
}

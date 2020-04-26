use std::str;
use std::{mem, slice};

use crate::marker::{marker_from_byte_fixx, Marker};

use super::reader::Reader;
use super::{DecoderOps, Reference, Result};

/// The default `DecoderOps` implementation.
///
pub struct Decoder<B> {
    reader: B,
    marker_cache: Option<(Marker, u8)>,
}

impl<'toy, B> Decoder<B>
where
    B: Reader<'toy>,
{
    pub fn new(reader: B) -> Decoder<B> {
        Decoder {
            reader,
            marker_cache: None,
        }
    }

    #[inline]
    fn get_marker_and_byte_core(&mut self) -> Result<(Marker, u8)> {
        let fb = self.get_byte()?;
        Ok(marker_from_byte_fixx(fb))
    }
}

impl<'toy, B> DecoderOps<'toy> for Decoder<B>
where
    B: Reader<'toy>,
{
    #[inline]
    fn remaining(&self) -> usize {
        self.reader.remaining()
    }

    #[inline]
    fn get<T: Sized>(&mut self) -> Result<&T> {
        let s = mem::size_of::<T>();
        assert!(self.remaining() >= s);
        let r = self
            .reader
            .get_bytes(s)
            .map(|x| unsafe { slice::from_raw_parts(x.as_ptr() as *const T, s) })?;
        Ok(&r[0])
    }

    #[inline]
    fn get_byte(&mut self) -> Result<u8> {
        assert!(self.remaining() >= 1);
        self.reader.get_byte()
    }

    #[inline]
    fn get_bytes<'a>(&'a mut self, len: usize) -> Result<Reference<'toy, 'a, [u8]>> {
        self.reader.get_bytes(len)
    }

    #[inline]
    fn discard(&mut self, len: usize) -> Result<()> {
        self.reader.discard(len)
    }

    fn decode_str<'a>(&'a mut self) -> Result<Reference<'toy, 'a, str>> {
        let len = self.decode_str_len()? as usize;
        match self.reader.get_bytes(len)? {
            Reference::Borrowed(b) => str::from_utf8(b)
                .map(Reference::Borrowed)
                .map_err(Into::into),
            Reference::Copied(c) => str::from_utf8(c).map(Reference::Copied).map_err(Into::into),
        }
    }

    #[inline]
    fn peek_marker(&mut self) -> Result<Marker> {
        self.marker_cache.map(|x| Ok(x.0)).unwrap_or_else(|| {
            let r = self.get_marker_and_byte_core()?;
            self.marker_cache = Some(r);
            Ok(r.0)
        })
    }

    #[inline]
    fn peek_marker_and_byte(&mut self) -> Result<(Marker, u8)> {
        self.marker_cache.map(Ok).unwrap_or_else(|| {
            let r = self.get_marker_and_byte_core()?;
            self.marker_cache = Some(r);
            Ok(r)
        })
    }

    #[inline]
    fn get_marker(&mut self) -> Result<Marker> {
        match self.marker_cache {
            Some(v) => {
                self.marker_cache = None;
                Ok(v.0)
            }
            None => {
                let r = self.get_marker_and_byte_core()?;
                Ok(r.0)
            }
        }
    }

    #[inline]
    fn get_marker_and_byte(&mut self) -> Result<(Marker, u8)> {
        match self.marker_cache {
            Some(v) => {
                self.marker_cache = None;
                Ok(v)
            }
            None => self.get_marker_and_byte_core(),
        }
    }
}

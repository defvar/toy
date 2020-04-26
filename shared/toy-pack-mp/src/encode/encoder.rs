use super::{EncoderOps, Result, Writer};

/// The default `EncoderOps` implementation.
///
pub struct Encoder<W> {
    pub writer: W,
}

impl<W> Encoder<W> {
    pub fn new(writer: W) -> Encoder<W> {
        Encoder { writer }
    }
}

impl<W> EncoderOps for Encoder<W>
where
    W: Writer,
{
    #[inline]
    fn remaining(&self) -> usize {
        self.writer.remaining()
    }

    #[inline]
    fn put<T: Sized>(&mut self, v: T) {
        self.writer.put_slice(&[v])
    }

    #[inline]
    fn put_slice<T: Sized>(&mut self, v: &[T]) {
        self.writer.put_slice(v)
    }

    #[inline]
    fn put_byte(&mut self, v: u8) -> Result<()> {
        self.writer.put_byte(v)
    }
}

use std::ops::Deref;
use std::str;

use toy_pack::deser::from_primitive::FromPrimitive;

use crate::marker::Marker;

use super::{DecodeError, Result};

pub enum Reference<'b, 'c, T: ?Sized + 'static> {
    Borrowed(&'b T),
    Copied(&'c T),
}

impl<'b, 'c, T: ?Sized + 'static> Deref for Reference<'b, 'c, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(c) => c,
        }
    }
}

/// Decode for msgpack.
///
pub trait DecoderOps<'toy> {
    fn remaining(&self) -> usize;

    fn get<T: Sized>(&mut self) -> Result<&T>;

    fn get_byte(&mut self) -> Result<u8>;

    fn get_bytes<'a>(&'a mut self, len: usize) -> Result<Reference<'toy, 'a, [u8]>>;

    fn discard(&mut self, len: usize) -> Result<()>;

    // msgpack //

    fn decode_str<'a>(&'a mut self) -> Result<Reference<'toy, 'a, str>>;

    /// Markers are checked and bytes read are cached.
    /// The next `get_marker` will return the value from the cached byte.
    /// If you want to check the marker but you do not want to advance the stream (nil check etc), use it.
    ///
    /// マーカーをチェックし、読みだしたバイトはキャッシュされます。
    /// 次回の`get_marker`ではキャッシュされたバイトから値が返されます。
    /// マーカーは確認したいが、ストリームを進めたくない場合（nil チェック等）で利用します。
    ///
    fn peek_marker(&mut self) -> Result<Marker>;

    /// Markers are checked and bytes read are cached.
    /// The next `get_marker` will return the value from the cached byte.
    /// If you want to check the marker but you do not want to advance the stream (nil check etc), use it.
    ///
    /// マーカーをチェックし、読みだしたバイトはキャッシュされます。
    /// 次回の`get_marker`ではキャッシュされたバイトから値が返されます。
    /// マーカーは確認したいが、ストリームを進めたくない場合（nil チェック等）で利用します。
    ///
    fn peek_marker_and_byte(&mut self) -> Result<(Marker, u8)>;

    fn get_marker(&mut self) -> Result<Marker>;

    fn get_marker_and_byte(&mut self) -> Result<(Marker, u8)>;

    fn decode_nil(&mut self) -> Result<()> {
        match self.get_marker()? {
            Marker::Nil => Ok(()),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_bool(&mut self) -> Result<bool> {
        match self.get_marker()? {
            Marker::True => Ok(true),
            Marker::False => Ok(false),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_u8(&mut self) -> Result<u8> {
        match self.get_marker()? {
            Marker::U8 => self.get_byte(),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_u16(&mut self) -> Result<u16> {
        match self.get_marker()? {
            Marker::U16 => self.get_u16(),
            other => Err(DecodeError::from(other)),
        }
    }

    #[inline]
    fn get_u16(&mut self) -> Result<u16> {
        Ok(u16::from_be(*self.get::<u16>()?))
    }

    fn decode_u32(&mut self) -> Result<u32> {
        match self.get_marker()? {
            Marker::U32 => self.get_u32(),
            other => Err(DecodeError::from(other)),
        }
    }

    #[inline]
    fn get_u32(&mut self) -> Result<u32> {
        Ok(u32::from_be(*self.get::<u32>()?))
    }

    fn decode_u64(&mut self) -> Result<u64> {
        match self.get_marker()? {
            Marker::U64 => Ok(u64::from_be(*self.get::<u64>()?)),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_i8(&mut self) -> Result<i8> {
        match self.get_marker()? {
            Marker::I8 => Ok(i8::from_be(*self.get::<i8>()?)),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_i16(&mut self) -> Result<i16> {
        match self.get_marker()? {
            Marker::I16 => Ok(i16::from_be(*self.get::<i16>()?)),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_i32(&mut self) -> Result<i32> {
        match self.get_marker()? {
            Marker::I32 => Ok(i32::from_be(*self.get::<i32>()?)),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_i64(&mut self) -> Result<i64> {
        match self.get_marker()? {
            Marker::I64 => Ok(i64::from_be(*self.get::<i64>()?)),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_f32(&mut self) -> Result<f32> {
        match self.get_marker()? {
            Marker::Float32 => {
                Ok(unsafe { *(&u32::from_be(*self.get::<u32>()?) as *const u32 as *const f32) })
            }
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_f64(&mut self) -> Result<f64> {
        match self.get_marker()? {
            Marker::Float64 => {
                Ok(unsafe { *(&u64::from_be(*self.get::<u64>()?) as *const u64 as *const f64) })
            }
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_integer<T: FromPrimitive>(&mut self) -> Result<T> {
        let r = match self.peek_marker_and_byte()? {
            (Marker::FixPos, fb) => {
                let _ = self.get_marker()?; //consume
                T::from_u8(fb)
            }
            (Marker::FixNeg, fb) => {
                let _ = self.get_marker()?; //consume
                T::from_i8(fb as i8)
            }
            (Marker::U8, _) => T::from_u8(self.decode_u8()?),
            (Marker::U16, _) => T::from_u16(self.decode_u16()?),
            (Marker::U32, _) => T::from_u32(self.decode_u32()?),
            (Marker::U64, _) => T::from_u64(self.decode_u64()?),
            (Marker::I8, _) => T::from_i8(self.decode_i8()?),
            (Marker::I16, _) => T::from_i16(self.decode_i16()?),
            (Marker::I32, _) => T::from_i32(self.decode_i32()?),
            (Marker::I64, _) => T::from_i64(self.decode_i64()?),
            (other, _) => return Err(DecodeError::from(other)),
        };
        r.ok_or_else(|| DecodeError::OutOfRange)
    }

    fn decode_array_len(&mut self) -> Result<u32> {
        match self.get_marker_and_byte()? {
            (Marker::FixArray, fb) => Ok(fb as u32),
            (Marker::Array16, _) => Ok(self.get_u16()? as u32),
            (Marker::Array32, _) => Ok(self.get_u32()? as u32),
            (other, _) => Err(DecodeError::from(other)),
        }
    }

    fn decode_map_len(&mut self) -> Result<u32> {
        match self.get_marker_and_byte()? {
            (Marker::FixMap, fb) => Ok(fb as u32),
            (Marker::Map16, _) => Ok(self.get_u16()? as u32),
            (Marker::Map32, _) => Ok(self.get_u32()? as u32),
            (other, _) => Err(DecodeError::from(other)),
        }
    }

    fn decode_bin_len(&mut self) -> Result<u32> {
        match self.get_marker()? {
            Marker::Bin8 => Ok(self.get_byte()? as u32),
            Marker::Bin16 => Ok(self.get_u16()? as u32),
            Marker::Bin32 => Ok(self.get_u32()? as u32),
            other => Err(DecodeError::from(other)),
        }
    }

    fn decode_str_len(&mut self) -> Result<u32> {
        match self.get_marker_and_byte()? {
            (Marker::FixStr, fb) => Ok(fb as u32),
            (Marker::Str8, _) => Ok(self.get_byte()? as u32),
            (Marker::Str16, _) => Ok(self.get_u16()? as u32),
            (Marker::Str32, _) => Ok(self.get_u32()? as u32),
            (other, _) => Err(DecodeError::from(other)),
        }
    }

    fn discard_next(&mut self) -> Result<()> {
        let mut c = 1;
        while c > 0 {
            let m = self.peek_marker()?;

            if m.is_map_type() {
                c += self.decode_map_len()? * 2;
            } else if m.is_array_type() {
                c += self.decode_array_len()?;
            } else if m.is_str_type() {
                let len = self.decode_str_len()? as usize;
                self.discard(len)?;
            } else if m.is_bin_type() {
                let len = self.decode_bin_len()? as usize;
                self.discard(len)?;
            } else {
                match self.get_marker()? {
                    Marker::True | Marker::False | Marker::Nil => (),
                    Marker::FixNeg | Marker::FixPos => (),
                    Marker::U8 | Marker::I8 => self.discard(1)?,
                    Marker::U16 | Marker::I16 => self.discard(2)?,
                    Marker::U32 | Marker::I32 | Marker::Float32 => self.discard(4)?,
                    Marker::U64 | Marker::I64 | Marker::Float64 => self.discard(8)?,
                    Marker::FixExt1 => self.discard(2)?,
                    Marker::FixExt2 => self.discard(3)?,
                    Marker::FixExt4 => self.discard(5)?,
                    Marker::FixExt8 => self.discard(9)?,
                    Marker::FixExt16 => self.discard(17)?,
                    Marker::Ext8 => unimplemented!(),
                    Marker::Ext16 => unimplemented!(),
                    Marker::Ext32 => unimplemented!(),
                    _ => (),
                };
            }
            c -= 1;
        }
        Ok(())
    }
}

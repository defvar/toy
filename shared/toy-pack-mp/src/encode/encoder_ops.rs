use crate::marker::{Marker, marker_to_byte};

use super::Result;

/// Encode for msgpack.
///
pub trait EncoderOps {
    fn remaining(&self) -> usize;

    fn put<T: Sized>(&mut self, v: T);

    fn put_slice<T: Sized>(&mut self, v: &[T]);

    fn put_byte(&mut self, v: u8) -> Result<()>;

    // msgpack //

    #[inline]
    fn put_marker(&mut self, marker: Marker) -> Result<()> {
        self.put_byte(marker_to_byte(marker, None))?;
        Ok(())
    }

    #[inline]
    fn put_marker_with_len(&mut self, marker: Marker, fix_len: u8) -> Result<()> {
        self.put_byte(marker_to_byte(marker, Some(fix_len)))?;
        Ok(())
    }

    fn encode_nil(&mut self) -> Result<()> {
        self.put_marker(Marker::Nil)?;
        Ok(())
    }

    fn encode_bool(&mut self, v: bool) -> Result<()> {
        if v {
            self.put_marker(Marker::True)
        } else {
            self.put_marker(Marker::False)
        }
    }

    fn encode_fix_pos(&mut self, v: u8) -> Result<()> {
        assert!(v < (1 << 7));
        self.put_byte(v)?;
        Ok(())
    }

    fn encode_u8(&mut self, v: u8) -> Result<()> {
        self.put_marker(Marker::U8)?;
        self.put::<u8>(v);
        Ok(())
    }

    fn encode_u16(&mut self, v: u16) -> Result<()> {
        self.put_marker(Marker::U16)?;
        self.put::<u16>(v.to_be());
        Ok(())
    }

    fn encode_u32(&mut self, v: u32) -> Result<()> {
        self.put_marker(Marker::U32)?;
        self.put::<u32>(v.to_be());
        Ok(())
    }

    fn encode_u64(&mut self, v: u64) -> Result<()> {
        self.put_marker(Marker::U64)?;
        self.put::<u64>(v.to_be());
        Ok(())
    }

    fn encode_fix_neg(&mut self, v: i8) -> Result<()> {
        assert!(-32 <= v && v < 0);
        self.put_byte(v as u8)?;
        Ok(())
    }

    fn encode_i8(&mut self, v: i8) -> Result<()> {
        self.put_marker(Marker::I8)?;
        self.put::<i8>(v);
        Ok(())
    }

    fn encode_i16(&mut self, v: i16) -> Result<()> {
        self.put_marker(Marker::I16)?;
        self.put::<i16>(v.to_be());
        Ok(())
    }

    fn encode_i32(&mut self, v: i32) -> Result<()> {
        self.put_marker(Marker::I32)?;
        self.put::<i32>(v.to_be());
        Ok(())
    }

    fn encode_i64(&mut self, v: i64) -> Result<()> {
        self.put_marker(Marker::I64)?;
        self.put::<i64>(v.to_be());
        Ok(())
    }

    fn encode_f32(&mut self, v: f32) -> Result<()> {
        self.put_marker(Marker::Float32)?;
        let n = unsafe { *(&v as *const f32 as *const u32) };
        self.put::<u32>(n.to_be());
        Ok(())
    }

    fn encode_f64(&mut self, v: f64) -> Result<()> {
        self.put_marker(Marker::Float64)?;
        let n = unsafe { *(&v as *const f64 as *const u64) };
        self.put::<u64>(n.to_be());
        Ok(())
    }

    fn encode_uint(&mut self, v: u64) -> Result<Marker> {
        if v < (1 << 7) {
            self.encode_fix_pos(v as u8).and(Ok(Marker::FixPos))
        } else if v < (1 << 8) {
            self.encode_u8(v as u8).and(Ok(Marker::U8))
        } else if v < (1 << 16) {
            self.encode_u16(v as u16).and(Ok(Marker::U16))
        } else if v < (1 << 32) {
            self.encode_u32(v as u32).and(Ok(Marker::U32))
        } else {
            self.encode_u64(v as u64).and(Ok(Marker::U64))
        }
    }

    fn encode_sint(&mut self, v: i64) -> Result<Marker> {
        match v {
            v if -(1 << 5) <= v && v < 0 => self.encode_fix_neg(v as i8).and(Ok(Marker::FixNeg)),
            v if -(1 << 7) <= v && v < -(1 << 5) => self.encode_i8(v as i8).and(Ok(Marker::I8)),
            v if -(1 << 15) <= v && v < -(1 << 7) => self.encode_i16(v as i16).and(Ok(Marker::I16)),
            v if -(1 << 31) <= v && v < -(1 << 15) => self.encode_i32(v as i32).and(Ok(Marker::I32)),
            v if v < -(1 << 31) => self.encode_i64(v).and(Ok(Marker::I64)),
            v if 0 <= v && v < (1 << 7) => self.encode_fix_pos(v as u8).and(Ok(Marker::FixPos)),
            v if v < (1 << 8) => self.encode_u8(v as u8).and(Ok(Marker::U8)),
            v if v < (1 << 16) => self.encode_u16(v as u16).and(Ok(Marker::U16)),
            v if v < (1 << 32) => self.encode_u32(v as u32).and(Ok(Marker::U32)),
            v => self.encode_u64(v as u64).and(Ok(Marker::U64)),
        }
    }

    fn encode_str(&mut self, v: &str) -> Result<()> {
        self.encode_str_len(v.len() as u32)?;
        self.put_slice(v.as_bytes());
        Ok(())
    }

    fn encode_str_len(&mut self, len: u32) -> Result<Marker> {
        if len < 32 {
            self.put_marker_with_len(Marker::FixStr, len as u8)
                .and(Ok(Marker::FixStr))
        } else if len < 256 {
            self.put_marker(Marker::Str8)
                .and(self.put_byte(len as u8))
                .and(Ok(Marker::Str8))
        } else if len < 65536 {
            self.put_marker(Marker::Str16)?;
            self.put::<u16>((len as u16).to_be());
            Ok(Marker::Str16)
        } else {
            self.put_marker(Marker::Str32)?;
            self.put::<u32>(len.to_be());
            Ok(Marker::Str32)
        }
    }

    fn encode_array_len(&mut self, len: u32) -> Result<Marker> {
        if len < 16 {
            self.put_marker_with_len(Marker::FixArray, len as u8)
                .and(Ok(Marker::FixArray))
        } else if len < (1 << 16) {
            self.put_marker(Marker::Array16)?;
            self.put::<u16>((len as u16).to_be());
            Ok(Marker::Array16)
        } else {
            self.put_marker(Marker::Array32)?;
            self.put::<u32>(len.to_be());
            Ok(Marker::Array32)
        }
    }

    fn encode_map_len(&mut self, len: u32) -> Result<Marker> {
        assert!(len > 0);

        if len < 16 {
            self.put_marker_with_len(Marker::FixMap, len as u8)
                .and(Ok(Marker::FixMap))
        } else if len < (1 << 16) {
            self.put_marker(Marker::Map16)?;
            self.put::<u16>((len as u16).to_be());
            Ok(Marker::Map16)
        } else {
            self.put_marker(Marker::Map32)?;
            self.put::<u32>(len.to_be());
            Ok(Marker::Map32)
        }
    }
}

#![allow(unreachable_patterns)]

lazy_static! {
  static ref MARKERS_FROM_BYTE: [Marker; 256] = {
      let mut buf = [Marker::Nil; 256];
      for b in 0..256 {
          buf[b] = Marker::from_u8(b as u8);
      }
      buf
  };
}

const FIX_STR_SIZE: u8 = 0x1f;
const FIX_ARRAY_SIZE: u8 = 0x0f;
const FIX_MAP_SIZE: u8 = 0x0f;

#[inline]
pub fn marker_from_byte(b: u8) -> Marker {
    MARKERS_FROM_BYTE[b as usize]
}

#[inline]
pub fn marker_from_byte_fixx(b: u8) -> (Marker, u8) {
    match MARKERS_FROM_BYTE[b as usize] {
        m @ Marker::FixPos => (m, b),
        m @ Marker::FixNeg => (m, b),
        m @ Marker::FixMap => (m, b & FIX_MAP_SIZE),
        m @ Marker::FixArray => (m, b & FIX_ARRAY_SIZE),
        m @ Marker::FixStr => (m, b & FIX_STR_SIZE),
        other => (other, b),
    }
}

#[inline]
pub fn marker_to_byte(marker: Marker, fix_len: Option<u8>) -> u8 {
    let len = fix_len.unwrap_or(0);
    match marker {
        Marker::FixPos => len,
        Marker::FixNeg => len,
        Marker::FixMap => 0x80 | (len & FIX_MAP_SIZE),
        Marker::FixArray => 0x90 | (len & FIX_ARRAY_SIZE),
        Marker::FixStr => 0xa0 | (len & FIX_STR_SIZE),
        _ => marker.to_u8(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Marker {
    FixPos = 0,
    FixNeg = 1,
    FixMap = 2,
    FixArray = 3,
    FixStr = 4,
    Nil = 5,
    False = 6,
    True = 7,
    Bin8 = 8,
    Bin16 = 9,
    Bin32 = 10,
    Ext8 = 11,
    Ext16 = 12,
    Ext32 = 13,
    Float32 = 14,
    Float64 = 15,
    U8 = 16,
    U16 = 17,
    U32 = 18,
    U64 = 19,
    I8 = 20,
    I16 = 21,
    I32 = 22,
    I64 = 23,
    FixExt1 = 24,
    FixExt2 = 25,
    FixExt4 = 26,
    FixExt8 = 27,
    FixExt16 = 28,
    Str8 = 29,
    Str16 = 30,
    Str32 = 31,
    Array16 = 32,
    Array32 = 33,
    Map16 = 34,
    Map32 = 35,
    Reserved = 201,
}

impl Marker {
    fn from_u8(b: u8) -> Marker {
        match b {
            0x00..=0x7f => Marker::FixPos,
            0xe0..=0xff => Marker::FixNeg,
            0x80..=0x8f => Marker::FixMap,
            0x90..=0x9f => Marker::FixArray,
            0xa0..=0xbf => Marker::FixStr,
            0xc0 => Marker::Nil,
            0xc1 => Marker::Reserved,
            0xc2 => Marker::False,
            0xc3 => Marker::True,
            0xc4 => Marker::Bin8,
            0xc5 => Marker::Bin16,
            0xc6 => Marker::Bin32,
            0xc7 => Marker::Ext8,
            0xc8 => Marker::Ext16,
            0xc9 => Marker::Ext32,
            0xca => Marker::Float32,
            0xcb => Marker::Float64,
            0xcc => Marker::U8,
            0xcd => Marker::U16,
            0xce => Marker::U32,
            0xcf => Marker::U64,
            0xd0 => Marker::I8,
            0xd1 => Marker::I16,
            0xd2 => Marker::I32,
            0xd3 => Marker::I64,
            0xd4 => Marker::FixExt1,
            0xd5 => Marker::FixExt2,
            0xd6 => Marker::FixExt4,
            0xd7 => Marker::FixExt8,
            0xd8 => Marker::FixExt16,
            0xd9 => Marker::Str8,
            0xda => Marker::Str16,
            0xdb => Marker::Str32,
            0xdc => Marker::Array16,
            0xdd => Marker::Array32,
            0xde => Marker::Map16,
            0xdf => Marker::Map32,
            _ => Marker::Nil,
        }
    }

    fn to_u8(&self) -> u8 {
        match *self {
            Marker::Nil => 0xc0,

            Marker::True => 0xc3,
            Marker::False => 0xc2,

            Marker::U8 => 0xcc,
            Marker::U16 => 0xcd,
            Marker::U32 => 0xce,
            Marker::U64 => 0xcf,

            Marker::I8 => 0xd0,
            Marker::I16 => 0xd1,
            Marker::I32 => 0xd2,
            Marker::I64 => 0xd3,

            Marker::Float32 => 0xca,
            Marker::Float64 => 0xcb,

            Marker::Str8 => 0xd9,
            Marker::Str16 => 0xda,
            Marker::Str32 => 0xdb,

            Marker::Bin8 => 0xc4,
            Marker::Bin16 => 0xc5,
            Marker::Bin32 => 0xc6,

            Marker::Array16 => 0xdc,
            Marker::Array32 => 0xdd,

            Marker::Map16 => 0xde,
            Marker::Map32 => 0xdf,

            Marker::FixExt1 => 0xd4,
            Marker::FixExt2 => 0xd5,
            Marker::FixExt4 => 0xd6,
            Marker::FixExt8 => 0xd7,
            Marker::FixExt16 => 0xd8,
            Marker::Ext8 => 0xc7,
            Marker::Ext16 => 0xc8,
            Marker::Ext32 => 0xc9,

            Marker::Reserved => 0xc1,
            _ => 0x00,
        }
    }

    pub fn is_array_type(&self) -> bool {
        match *self {
            Marker::FixArray | Marker::Array16 | Marker::Array32 => true,
            _ => false,
        }
    }

    pub fn is_map_type(&self) -> bool {
        match *self {
            Marker::FixMap | Marker::Map16 | Marker::Map32 => true,
            _ => false,
        }
    }

    pub fn is_str_type(&self) -> bool {
        match *self {
            Marker::FixStr | Marker::Str8 | Marker::Str16 | Marker::Str32 => true,
            _ => false,
        }
    }

    pub fn is_bin_type(&self) -> bool {
        match *self {
            Marker::Bin8 | Marker::Bin16 | Marker::Bin32 => true,
            _ => false,
        }
    }
}

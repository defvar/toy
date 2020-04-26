use toy_pack_mp::{decoder_from_slice, marker::Marker, DecodeError, DecoderOps};

macro_rules! decode_pass {
    ($name: ident, $method: ident, $bytes: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let buf: &[u8] = $bytes;
            let mut decoder = decoder_from_slice(buf);
            let actual = decoder.$method().unwrap();
            assert_eq!($expected, actual);
        }
    };
}

macro_rules! decode_invalid_type {
    ($name: ident, $method: ident) => {
        #[test]
        fn $name() {
            let buf: &[u8] = &[0xc0];
            let mut decoder = decoder_from_slice(buf);
            match decoder.$method() {
                Err(DecodeError::InvalidType { .. }) => (),
                other => panic!("unexpected result: {:?}", other),
            }
        }
    };
}

decode_pass!(decode_u8, decode_u8, &[0xcc, 0xff], u8::max_value());

decode_pass!(
    decode_u16,
    decode_u16,
    &[0xcd, 0xff, 0xff],
    u16::max_value()
);
decode_pass!(decode_u16_endian, decode_u16, &[0xcd, 0x00, 0x01], 1);

decode_pass!(
    decode_u32,
    decode_u32,
    &[0xce, 0xff, 0xff, 0xff, 0xff],
    u32::max_value()
);
decode_pass!(
    decode_u32_endian,
    decode_u32,
    &[0xce, 0x00, 0x00, 0x00, 0x01],
    1
);

decode_pass!(
    decode_u64,
    decode_u64,
    &[0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    u64::max_value()
);
decode_pass!(
    decode_u64_endian,
    decode_u64,
    &[0xcf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    1
);

decode_pass!(decode_i8, decode_i8, &[0xd0, 0x7f], i8::max_value());

decode_pass!(
    decode_i16,
    decode_i16,
    &[0xd1, 0x7f, 0xff],
    i16::max_value()
);
decode_pass!(decode_i16_endian, decode_i16, &[0xd1, 0x00, 0x01], 1);

decode_pass!(
    decode_i32,
    decode_i32,
    &[0xd2, 0x7f, 0xff, 0xff, 0xff],
    i32::max_value()
);
decode_pass!(
    decode_i32_endian,
    decode_i32,
    &[0xd2, 0x00, 0x00, 0x00, 0x01],
    1
);

decode_pass!(
    decode_i64,
    decode_i64,
    &[0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    i64::max_value()
);
decode_pass!(
    decode_i64_endian,
    decode_i64,
    &[0xd3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    1
);

decode_invalid_type!(decode_u8_invalid_type, decode_u8);
decode_invalid_type!(decode_u16_invalid_type, decode_u16);
decode_invalid_type!(decode_u32_invalid_type, decode_u32);
decode_invalid_type!(decode_u64_invalid_type, decode_u64);

decode_invalid_type!(decode_i8_invalid_type, decode_i8);
decode_invalid_type!(decode_i16_invalid_type, decode_i16);
decode_invalid_type!(decode_i32_invalid_type, decode_i32);
decode_invalid_type!(decode_i64_invalid_type, decode_i64);

#[test]
fn decode_fix_pos() {
    let buf: &[u8] = &[0x7f];
    let mut decoder = decoder_from_slice(buf);
    let a: u8 = decoder.decode_integer().unwrap();
    assert_eq!(127, a);
}

#[test]
fn decode_fix_neg() {
    let buf: &[u8] = &[0xe0];
    let mut decoder = decoder_from_slice(buf);
    let a: i8 = decoder.decode_integer().unwrap();
    assert_eq!(-32, a);
}

#[test]
fn peek_and_decode_u8() {
    let buf: &[u8] = &[0xcc, 0xff];
    let mut decoder = decoder_from_slice(buf);
    let (marker, fb) = decoder.peek_marker_and_byte().unwrap();

    assert_eq!(Marker::U8, marker);
    assert_eq!(0xcc, fb);

    // second try, get from cache
    let (marker, fb) = decoder.peek_marker_and_byte().unwrap();

    assert_eq!(Marker::U8, marker);
    assert_eq!(0xcc, fb);

    let a: u8 = decoder.decode_integer().unwrap();
    assert_eq!(u8::max_value(), a);
}

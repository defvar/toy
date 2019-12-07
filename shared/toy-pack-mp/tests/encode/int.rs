use toy_pack_mp::{encoder_from_writer, EncoderOps};

macro_rules! encode_pass {
    ($name: ident, $method: ident, $value: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let mut vec: Vec<u8> = Vec::new();
            let mut encoder = encoder_from_writer(&mut vec);
            encoder.$method($value).unwrap();
            assert_eq!($expected, vec);
        }
    };
}

encode_pass!(encode_u8,         encode_u8,  u8::max_value(), vec![0xcc, 0xff]);

encode_pass!(encode_u16,        encode_u16, u16::max_value(), vec![0xcd, 0xff, 0xff]);
encode_pass!(encode_u16_endian, encode_u16, 1,                vec![0xcd, 0x00, 0x01]);

encode_pass!(encode_u32,        encode_u32, u32::max_value(), vec![0xce, 0xff, 0xff, 0xff, 0xff]);
encode_pass!(encode_u32_endian, encode_u32, 1,                vec![0xce, 0x00, 0x00, 0x00, 0x01]);

encode_pass!(encode_u64,        encode_u64, u64::max_value(), vec![0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
encode_pass!(encode_u64_endian, encode_u64, 1,                vec![0xcf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);

encode_pass!(encode_i8,         encode_i8,  i8::max_value(), vec![0xd0, 0x7f]);

encode_pass!(encode_i16,        encode_i16, i16::max_value(), vec![0xd1, 0x7f, 0xff]);
encode_pass!(encode_i16_endian, encode_i16, 1,                vec![0xd1, 0x00, 0x01]);

encode_pass!(encode_i32,        encode_i32, i32::max_value(), vec![0xd2, 0x7f, 0xff, 0xff, 0xff]);
encode_pass!(encode_i32_endian, encode_i32, 1,                vec![0xd2, 0x00, 0x00, 0x00, 0x01]);

encode_pass!(encode_i64,        encode_i64, i64::max_value(), vec![0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
encode_pass!(encode_i64_endian, encode_i64, 1,                vec![0xd3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);

#[test]
fn encode_fix_pos() {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    encoder.encode_fix_pos(127).unwrap();

    assert_eq!(vec![0x7f], vec);
}

#[test]
fn encode_fix_neg() {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    encoder.encode_fix_neg(-32).unwrap();

    assert_eq!(vec![0xe0], vec);
}

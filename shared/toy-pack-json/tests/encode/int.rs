use toy_pack_json::encoder_from_writer;

macro_rules! encode_pass {
    ($name: ident, $method: ident, $value: expr, $expected: literal) => {
        #[test]
        fn $name() {
            let mut buf = Vec::new();
            let mut encoder = encoder_from_writer(&mut buf);
            encoder.$method($value).unwrap();
            assert_eq!(std::str::from_utf8(buf.as_slice()).unwrap(), $expected);
        }
    };
}

encode_pass!(encode_u8, write_u8, 1u8, "1");
encode_pass!(encode_u16, write_u16, 1u16, "1");
encode_pass!(encode_u32, write_u32, 1u32, "1");
encode_pass!(encode_u64, write_u64, 1u64, "1");

encode_pass!(encode_i8, write_i8, -1i8, "-1");
encode_pass!(encode_i16, write_i16, -1i16, "-1");
encode_pass!(encode_i32, write_i32, -1i32, "-1");
encode_pass!(encode_i64, write_i64, -1i64, "-1");

encode_pass!(encode_f32, write_f32, 1.2, "1.2");
encode_pass!(encode_f64, write_f64, 3.4, "3.4");

use toy_pack_json::{decoder_from_slice, ParseNumber};

#[test]
fn decode_float() {
    let buf: &[u8] = b"12.3";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_number().unwrap();
    assert_eq!(ParseNumber::F64(12.3), a);
}

#[test]
fn decode_float_negative() {
    let buf: &[u8] = b"-12.3";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_number().unwrap();
    assert_eq!(ParseNumber::F64(-12.3), a);
}

#[test]
fn decode_float_exponent() {
    let data: &[(&str, &[u8], ParseNumber)] = &[
        ("common", b"12.3e5", ParseNumber::F64(1230000f64)),
        ("big E", b"12.3E5", ParseNumber::F64(1230000f64)),
        ("negative", b"-12.3e5", ParseNumber::F64(-1230000f64)),
        (
            "negative exponent",
            b"12.3e-5",
            ParseNumber::F64(0.000123f64),
        ),
        (".0", b"3.0e5", ParseNumber::F64(300000f64)),
    ];

    for (name, d, e) in data {
        let mut decoder = decoder_from_slice(d);
        let a = decoder.decode_number().unwrap();
        assert_eq!(*e, a, "{}", name);
    }
}

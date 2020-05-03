use toy_pack_json::{decoder_from_slice, ParseNumber};

#[test]
fn decode_integer() {
    let buf: &[u8] = b"123";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_number().unwrap();
    assert_eq!(ParseNumber::U64(123), a);
}

#[test]
fn decode_integer_negative() {
    let buf: &[u8] = b"-123";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_number().unwrap();
    assert_eq!(ParseNumber::I64(-123), a);
}

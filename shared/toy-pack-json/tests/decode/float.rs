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

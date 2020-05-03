use toy_pack_json::decoder_from_slice;

#[test]
fn decode_float() {
    let buf: &[u8] = b"12.3";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_f64().unwrap();
    assert_eq!(12.3, a);
}

#[test]
fn decode_float_negative() {
    let buf: &[u8] = b"-12.3";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_f64().unwrap();
    assert_eq!(-12.3, a);
}

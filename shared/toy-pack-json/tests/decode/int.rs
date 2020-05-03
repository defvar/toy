use toy_pack_json::decoder_from_slice;

#[test]
fn decode_integer() {
    let buf: &[u8] = b"123";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_u64().unwrap();
    assert_eq!(123, a);
}

#[test]
fn decode_integer_negative() {
    let buf: &[u8] = b"-123";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_i64().unwrap();
    assert_eq!(-123, a);
}

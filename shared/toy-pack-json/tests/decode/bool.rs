use toy_pack_json::decoder_from_slice;

#[test]
fn decode_true() {
    let buf: &[u8] = b"true";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bool().unwrap();
    assert_eq!(true, a);
}

#[test]
fn decode_false() {
    let buf: &[u8] = b"false";
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bool().unwrap();
    assert_eq!(false, a);
}

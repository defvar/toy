use toy_pack_json::decoder_from_slice;

#[test]
fn decode_str() {
    let s: &[u8] = b"\"st\\nring\"";
    let mut buf = Vec::new();
    let mut decoder = decoder_from_slice(s);
    let d = decoder.decode_str(&mut buf).unwrap();
    assert_eq!("st\nring", &*d);
}

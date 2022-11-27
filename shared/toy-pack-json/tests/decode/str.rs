use toy_pack_json::decoder_from_slice;

#[test]
fn decode_str() {
    let s: &[u8] = b"\"st\\nring\"";
    let mut buf = Vec::new();
    let mut decoder = decoder_from_slice(s);
    let d = decoder.decode_str(&mut buf).unwrap();
    assert_eq!("st\nring", &*d);
}

#[test]
fn decode_str_escape_unicode_codepoint() {
    let s: &[u8] = b"\"\\u003caiueo\\u003e\"";
    let mut buf = Vec::new();
    let mut decoder = decoder_from_slice(s);
    let d = decoder.decode_str(&mut buf).unwrap();
    assert_eq!("<aiueo>", &*d);
}

#[test]
fn decode_str_escape_surrogate() {
    let s: &[u8] = b"\"\\uD834\\uDD1E\"";
    let mut buf = Vec::new();
    let mut decoder = decoder_from_slice(s);
    let d = decoder.decode_str(&mut buf).unwrap();
    assert_eq!("𝄞", &*d);
}

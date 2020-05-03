use toy_pack_json::decoder_from_slice;

#[test]
fn peek_until_basic() {
    let buf: &[u8] = b"  a";
    let mut decoder = decoder_from_slice(buf);
    let r = decoder.peek_until().unwrap().unwrap();
    assert_eq!(b'a', r);
}

#[test]
fn peek_until_no_skip() {
    let buf: &[u8] = b"a";
    let mut decoder = decoder_from_slice(buf);
    let r = decoder.peek_until().unwrap().unwrap();
    assert_eq!(b'a', r);
}

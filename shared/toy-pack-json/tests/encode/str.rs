use toy_pack_json::encoder_from_writer;

#[test]
fn encode_basic() {
    let mut buf = Vec::new();
    let mut encoder = encoder_from_writer(&mut buf);
    encoder.write_string("abc").unwrap();
    assert_eq!(std::str::from_utf8(buf.as_slice()).unwrap(), "abc");
}

#[test]
fn encode_escape_n() {
    let mut buf = Vec::new();
    let mut encoder = encoder_from_writer(&mut buf);
    encoder.write_string("a\nbc").unwrap();
    assert_eq!(std::str::from_utf8(buf.as_slice()).unwrap(), "a\\nbc");
}

use toy_pack_json::encoder_from_writer;

#[test]
fn encode_true() {
    let mut buf = Vec::new();
    let mut encoder = encoder_from_writer(&mut buf);
    encoder.write_bool(true).unwrap();
    assert_eq!(std::str::from_utf8(buf.as_slice()).unwrap(), "true");
}

#[test]
fn encode_false() {
    let mut buf = Vec::new();
    let mut encoder = encoder_from_writer(&mut buf);
    encoder.write_bool(false).unwrap();
    assert_eq!(std::str::from_utf8(buf.as_slice()).unwrap(), "false");
}

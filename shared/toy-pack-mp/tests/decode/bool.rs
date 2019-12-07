use toy_pack_mp::{decoder_from_slice, DecoderOps};

#[test]
fn decode_true() {
    let buf: &[u8] = &[0xc3];
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bool().unwrap();
    assert_eq!(true, a);
}

#[test]
fn decode_false() {
    let buf: &[u8] = &[0xc2];
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bool().unwrap();
    assert_eq!(false, a);
}

use toy_pack_mp::{decoder_from_slice, DecoderOps};

#[test]
fn decode_bin8_min_len() {
    let buf: &[u8] = &[0xc4, 0x00];
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bin_len().unwrap();
    assert_eq!(0, a);
}

#[test]
fn decode_bin8_max_len() {
    let buf: &[u8] = &[0xc4, 0xff];
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bin_len().unwrap();
    assert_eq!((u8::max_value() as u32), a);
}

#[test]
fn decode_bin16_max_len() {
    let buf: &[u8] = &[0xc5, 0xff, 0xff];
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bin_len().unwrap();
    assert_eq!((u16::max_value() as u32), a);
}

#[test]
fn decode_bin32_max_len() {
    let buf: &[u8] = &[0xc6, 0xff, 0xff, 0xff, 0xff];
    let mut decoder = decoder_from_slice(buf);
    let a = decoder.decode_bin_len().unwrap();
    assert_eq!(u32::max_value(), a);
}

pub fn char_to_u8(v: char) -> u8 {
    let mut dest = [0u8; 4];
    let _ = v.encode_utf8(&mut dest);
    dest[0]
}

use toy_pack_mp::marker::{marker_from_byte, marker_from_byte_fixx, marker_to_byte, Marker};

#[test]
fn from_byte() {
    assert_eq!(Marker::Nil, marker_from_byte(0xc0));
    assert_eq!((Marker::FixPos, 1), marker_from_byte_fixx(0x01));
    assert_eq!(
        (Marker::FixMap, 2),
        marker_from_byte_fixx(0x80 | (2 & 0x0f))
    );
    assert_eq!(
        (Marker::FixArray, 2),
        marker_from_byte_fixx(0x90 | (2 & 0x0f))
    );
    assert_eq!(
        (Marker::FixStr, 2),
        marker_from_byte_fixx(0xa0 | (2 & 0x1f))
    );
}

#[test]
fn to_byte() {
    assert_eq!(0xc0, marker_to_byte(Marker::Nil, None));
    assert_eq!(0x01, marker_to_byte(Marker::FixPos, Some(1)));
    assert_eq!(0x80 | (2 & 0x0f), marker_to_byte(Marker::FixMap, Some(2)));
    assert_eq!(0x90 | (2 & 0x0f), marker_to_byte(Marker::FixArray, Some(2)));
    assert_eq!(0xa0 | (2 & 0x1f), marker_to_byte(Marker::FixStr, Some(2)));
}

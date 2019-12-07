use toy_pack_mp::{pack, unpack};

#[test]
fn string_values() {
    let data: &[(String, usize)] = &[
        ("".to_owned(), 1),
        ("a".repeat(30), 30 + 1),
        ("a".repeat(31), 31 + 1),
        ("a".repeat(32), 32 + 2),

        ("a".repeat(255), 255 + 2),
        ("a".repeat(256), 256 + 3),
        ("a".repeat(257), 257 + 3),

        ("a".repeat(65535), 65535 + 3),
        ("a".repeat(65536), 65536 + 5),
        ("a".repeat(65537), 65537 + 5),
    ];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected = unpack::<String>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

#[test]
fn str_values() {
    let s = "a";
    let vec = pack(&s).unwrap();
    let expected = unpack::<&str>(&vec).unwrap();

    assert_eq!(2, vec.len());
    assert_eq!(s, expected);
}

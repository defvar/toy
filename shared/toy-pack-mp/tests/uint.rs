use toy_pack_mp::{pack, unpack};

#[test]
fn uint_values() {
    let data: &[(u64, usize)] = &[
        (0, 1),
        (1, 1),
        (i8::MAX as u64, 1),
        (128, 2),
        (255, 2),
        (256, 3),
        (i16::MAX as u64, 3),
        (u16::MAX as u64, 3),
        (65536, 5),
        (i32::MAX as u64, 5),
        (u32::MAX as u64, 5),
        (4294967296, 9),
    ];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected: u64 = unpack::<u64>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

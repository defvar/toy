use toy_pack_mp::{pack, unpack};

#[test]
fn sint_values() {
    let data: &[(i64, usize)] = &[
        (i64::MIN, 9),
        (-2147483649, 9),
        (i32::MIN as i64, 5),
        (-32769, 5),
        (i16::MIN as i64, 3),
        (-129, 3),
        (i8::MIN as i64, 2),
        (-33, 2),
        (-32, 1),
        (0, 1),
        (1, 1),
        (i8::MAX as i64, 1),
        (128, 2),
        (255, 2),
        (256, 3),
        (i16::MAX as i64, 3),
        (u16::MAX as i64, 3),
        (65536, 5),
        (i32::MAX as i64, 5),
        (u32::MAX as i64, 5),
        (4294967296, 9),
    ];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected = unpack::<i64>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

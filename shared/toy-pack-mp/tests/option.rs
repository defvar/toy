use toy_pack_mp::{pack, unpack};

#[test]
fn option_values() {
    let data: &[(Option<u32>, usize)] = &[
        (Some(1u32), 1),
        (None, 1), //nil marker byte = 1
    ];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected: Option<u32> = unpack::<Option<u32>>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

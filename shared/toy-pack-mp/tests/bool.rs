use toy_pack_mp::{pack, unpack};

#[test]
fn bool_values() {
    let data: &[(bool, usize)] = &[
        (false, 1),
        (true, 1),
    ];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected = unpack::<bool>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

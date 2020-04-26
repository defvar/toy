use std::path;

use toy_pack_mp::{pack, unpack};

#[test]
fn path_buf_values() {
    let data: &[(path::PathBuf, usize)] = &[(path::PathBuf::from("a"), 2)];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected = unpack::<path::PathBuf>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

#[test]
fn path_values() {
    let data: &[(&path::Path, usize)] = &[(path::Path::new("a"), 2)];

    for (d, l) in data {
        let vec = pack(d).unwrap();
        let expected = unpack::<&path::Path>(&vec).unwrap();

        assert_eq!(*l, vec.len());
        assert_eq!(*d, expected);
    }
}

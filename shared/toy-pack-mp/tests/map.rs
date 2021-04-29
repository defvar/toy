use std::collections::HashMap;

use toy_pack_mp::{pack, unpack};

#[test]
fn hash_map_values() {
    let mut src: HashMap<u32, u32> = HashMap::new();
    src.insert(1, 10);
    src.insert(2, 20);
    src.insert(3, 30);

    let dest = pack(&src).unwrap();
    let r = unpack::<HashMap<u32, u32>>(&dest).unwrap();

    assert_eq!(r, src);
}

#[test]
fn hash_map_16_values() {
    let mut src: HashMap<u32, u32> = HashMap::new();

    for i in 0..17 {
        src.insert(i, i);
    }

    let dest = pack(&src).unwrap();
    let r = unpack::<HashMap<u32, u32>>(&dest).unwrap();

    assert_eq!(r, src);
}

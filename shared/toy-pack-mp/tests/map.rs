use std::collections::HashMap;
use toy_pack_derive::*;
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
fn hash_map_value_if_none() {
    #[derive(Debug, Clone, PartialEq, Pack, Unpack)]
    #[toy(ignore_pack_if_none)]
    struct Test {
        b: Option<u32>,
    }

    let mut src: HashMap<String, Test> = HashMap::new();
    src.insert("a".to_string(), Test { b: None });
    src.insert("b".to_string(), Test { b: None });

    let dest = pack(&src).unwrap();
    let r = unpack::<HashMap<String, Test>>(&dest).unwrap();

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

use toy_pack_mp::{pack, unpack};

#[test]
fn vec_values() {
    let mut src: Vec<u32> = Vec::with_capacity(16);
    src.push(1);
    src.push(2);
    src.push(3);

    let dest = pack(&src).unwrap();
    let r = unpack::<Vec<u32>>(&dest).unwrap();

    assert_eq!(r, src);
}

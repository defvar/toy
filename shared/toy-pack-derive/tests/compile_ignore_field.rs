use toy_pack_derive::*;

#[test]
fn ignore_field() {
    let expected = Data { v_u32: 1, i: true };

    let vec = toy_pack_mp::pack(&expected).unwrap();
    let actual = toy_pack_mp::unpack::<Data>(vec.as_slice()).unwrap();

    assert_eq!(expected, actual);
}

#[derive(Pack, Unpack, Debug, PartialEq)]
struct Data {
    v_u32: u32,
    #[toy(ignore, default = true)]
    i: bool,
}

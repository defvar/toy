use toy_pack_derive::*;
use toy_pack_json::unpack;

#[test]
fn de_unit_variant() {
    #[derive(Debug, PartialEq, Unpack)]
    enum Test {
        A,
        B,
    }
    let json = "\"B\"";
    let r = unpack::<Test>(json.as_bytes()).unwrap();

    assert_eq!(r, Test::B);
}

#[test]
fn de_tuple_variant() {
    #[derive(Debug, PartialEq, Unpack)]
    enum Test {
        A(u32, u32, u32),
        B(u8, u8),
    }
    let json = "{ \"B\" : [5, 6] }";
    let r = unpack::<Test>(json.as_bytes()).unwrap();

    assert_eq!(r, Test::B(5, 6));
}

#[test]
fn de_new_type_variant() {
    #[derive(Debug, PartialEq, Unpack)]
    enum Test {
        A(u32),
        B(u8),
    }
    let json = "{ \"B\" : 5 }";
    let r = unpack::<Test>(json.as_bytes()).unwrap();

    assert_eq!(r, Test::B(5));
}

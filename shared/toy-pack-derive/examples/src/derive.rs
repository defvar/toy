use toy_pack_derive::*;
use toy_pack_mp::{pack, unpack};

#[derive(Pack, Unpack, Schema, Debug, PartialEq)]
struct Dum<'a> {
    #[toy(rename = "u32")]
    v_u32: u32,
    v_string: String,
    v_borrowed_str: &'a str,
    v_option: Option<u8>,
    v_test_enum: TestEnum,
}

#[derive(Eq, PartialEq, Debug, Pack, Unpack, Schema)]
enum TestEnum {
    //unit variant
    #[toy(rename = "Variant_A")]
    A,

    //newtype variant
    B(u32),

    //tuple variant
    C(u32, u32),
}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::A
    }
}

fn main() {
    let mut src: Vec<Dum> = Vec::new();
    let dum1 = Dum {
        v_u32: 1,
        v_string: "a".to_owned(),
        v_borrowed_str: "b",
        v_option: None,
        v_test_enum: TestEnum::B(1),
    };

    let dum2 = Dum {
        v_u32: 2,
        v_string: "a".to_owned(),
        v_borrowed_str: "b",
        v_option: None,
        v_test_enum: TestEnum::B(1),
    };

    src.push(dum1);
    src.push(dum2);

    let vec = pack(&src).unwrap();
    println!("{:?}", vec);

    let r = unpack::<Vec<Dum>>(vec.as_slice()).unwrap();
    println!("{:?}", r);
}

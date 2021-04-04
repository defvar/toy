use toy_pack_derive::*;

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

    //struct variant
    D {
        id: u64,
        name: String,
    },
}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::A
    }
}

fn main() {
    println!("hello");
}

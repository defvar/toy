use toy_core::data::{self, Value};
use toy_core::{map_value, seq_value};
use toy_pack_derive::*;

#[test]
fn ser_tuple_variant() {
    #[derive(Pack, Unpack, PartialEq, Debug)]
    enum Variant {
        One(u32),
        Two(u32, u32),
    }
    let v = Variant::Two(1, 2);
    let r = data::pack(v).unwrap();
    let expected = map_value! {
        "Two" => seq_value! [1u32, 2u32],
    };
    assert_eq!(r, expected);
}

#[test]
fn ser_struct() {
    let src = Dum {
        v_u8: 8,
        v_i8_opt: Some(80),
        v_f32: 3.2,
        v_f64: 6.4,
        name: "aiueo".to_string(),
        vec: vec![Value::from(0u8), Value::from(1u8), Value::from(2u8)],
        inner: Inner { v_u8: 8 },
        terminator: Terminator::CRLF,
    };

    let expected = map_value! {
        "v_u8" => 8u8,
        "v_i8_opt" => Some(80i8),
        "v_f32" => 3.2f32,
        "v_f64" => 6.4f64,
        "name" => "aiueo",
        "vec" => seq_value! [0u8, 1u8, 2u8],
        "inner" => map_value! {
            "v_u8" => 8u8
        },
        "terminator" => "CRLF"
    };

    let dest = data::pack(src).unwrap();
    assert_eq!(dest, expected);
}

#[derive(Debug, PartialEq, Pack)]
struct Dum {
    v_u8: u8,
    v_i8_opt: Option<i8>,
    v_f32: f32,
    v_f64: f64,
    name: String,
    vec: Vec<Value>,
    inner: Inner,
    terminator: Terminator,
}

#[derive(Debug, PartialEq, Default, Pack)]
struct Inner {
    v_u8: u8,
}

#[derive(Debug, Pack, PartialEq)]
enum Terminator {
    CRLF,
    LF,
}

impl std::default::Default for Terminator {
    fn default() -> Self {
        Terminator::LF
    }
}

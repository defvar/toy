use core::time::Duration;
use toy_core::data::{self, Map, Value};
use toy_core::{map_value, seq_value};
use toy_pack_derive::*;

macro_rules! pass_de_integer {
    ($func: ident, $t: ident,  $expected: expr) => {
        #[test]
        fn $func() {
            assert_eq!($expected, data::unpack::<$t>(Value::from(1i8)).unwrap());
            assert_eq!($expected, data::unpack::<$t>(Value::from(1i16)).unwrap());
            assert_eq!($expected, data::unpack::<$t>(Value::from(1i32)).unwrap());
            assert_eq!($expected, data::unpack::<$t>(Value::from(1i64)).unwrap());

            assert_eq!($expected, data::unpack::<$t>(Value::from(1u8)).unwrap());
            assert_eq!($expected, data::unpack::<$t>(Value::from(1u16)).unwrap());
            assert_eq!($expected, data::unpack::<$t>(Value::from(1u32)).unwrap());
            assert_eq!($expected, data::unpack::<$t>(Value::from(1u64)).unwrap());
        }
    };
}

pass_de_integer!(de_u8, u8, 1u8);
pass_de_integer!(de_u16, u16, 1u16);
pass_de_integer!(de_u32, u32, 1u32);
pass_de_integer!(de_u64, u64, 1u64);

pass_de_integer!(de_i8, i8, 1i8);
pass_de_integer!(de_i16, i16, 1i16);
pass_de_integer!(de_i32, i32, 1i32);
pass_de_integer!(de_i64, i64, 1i64);

#[test]
fn de_timestamp() {
    let src = Duration::new(3, 4);
    let dest = data::unpack::<Duration>(Value::from(src)).unwrap();
    assert_eq!(src, dest);
}

#[test]
fn de_tuple_variant() {
    #[derive(Pack, UnPack, PartialEq, Debug)]
    enum Variant {
        One(u32),
        Two(u32, u32),
    }
    let v = map_value! {
        "Two" => seq_value! [1u32, 2u32],
    };
    let r = data::unpack::<Variant>(v).unwrap();
    assert_eq!(r, Variant::Two(1, 2));
}

#[test]
fn de_struct() {
    let expected = Dum {
        v_u8: 8,
        v_u16: 16,
        v_u32: 32,
        v_u64: 64,
        v_i8_opt: Some(80),
        v_f32: 3.2,
        v_f64: 6.4,
        name: "aiueo".to_string(),
        vec: vec![Value::from(0u8), Value::from(1u8), Value::from(2u8)],
        inner: Inner { v_u8: 8 },
        terminator: Terminator::CRLF,
        terminator_from_str: Terminator::CRLF,
    };

    // inner struct
    let mut inner = Map::new();
    inner.insert("v_u8".to_string(), Value::from(expected.inner.v_u8));

    // enum
    let mut terminator = Map::new();
    terminator.insert("CRLF".to_string(), Value::None);

    // struct
    let mut map = Map::new();
    map.insert("v_u8".to_string(), Value::from(expected.v_u8));
    map.insert("v_u16".to_string(), Value::from(expected.v_u16));
    map.insert("v_u32".to_string(), Value::from(expected.v_u32));
    map.insert("v_u64".to_string(), Value::from(expected.v_u64));
    map.insert("v_i8_opt".to_string(), Value::from(expected.v_i8_opt));
    map.insert("v_f32".to_string(), Value::from(expected.v_f32));
    map.insert("v_f64".to_string(), Value::from(expected.v_f64));
    map.insert("name".to_string(), Value::from(expected.name.clone()));
    map.insert("vec".to_string(), Value::from(expected.vec.clone()));
    map.insert("inner".to_string(), Value::from(inner.clone()));
    map.insert("terminator".to_string(), Value::from(terminator));
    map.insert("terminator_from_str".to_string(), Value::from("CRLF"));

    let v = Value::from(map);
    let v = data::unpack::<Dum>(v).unwrap();
    assert_eq!(v, expected);
}

#[derive(Debug, PartialEq, UnPack)]
struct Dum {
    v_u8: u8,
    v_u16: u16,
    v_u32: u32,
    v_u64: u64,
    v_i8_opt: Option<i8>,
    v_f32: f32,
    v_f64: f64,
    name: String,
    vec: Vec<Value>,
    inner: Inner,
    terminator: Terminator,
    terminator_from_str: Terminator,
}

#[derive(Debug, PartialEq, Default, UnPack)]
struct Inner {
    v_u8: u8,
}

#[derive(Debug, UnPack, PartialEq)]
enum Terminator {
    CRLF,
    LF,
}

impl std::default::Default for Terminator {
    fn default() -> Self {
        Terminator::LF
    }
}

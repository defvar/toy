use std::collections::HashMap;
use toy_core::data::{self, Value};
use toy_pack_derive::*;

#[test]
fn de_struct() {
    let src = Dum {
        v_u8: 8,
        v_u16: 16,
        v_u32: 32,
        v_u64: 64,
        v_f32: 3.2,
        v_f64: 6.4,
        name: "aiueo".to_string(),
        vec: vec![Value::from(0u8), Value::from(1u8), Value::from(2u8)],
        inner: Inner { v_u8: 8 },
        terminator: Terminator::CRLF,
    };

    // inner struct
    let mut inner = HashMap::new();
    inner.insert("v_u8".to_string(), Value::from(src.inner.v_u8));

    // enum
    let mut terminator = HashMap::new();
    terminator.insert("CRLF".to_string(), Value::None);

    // struct
    let mut map = HashMap::new();
    map.insert("v_u8".to_string(), Value::from(src.v_u8));
    map.insert("v_u16".to_string(), Value::from(src.v_u16));
    map.insert("v_u32".to_string(), Value::from(src.v_u32));
    map.insert("v_u64".to_string(), Value::from(src.v_u64));
    map.insert("v_f32".to_string(), Value::from(src.v_f32));
    map.insert("v_f64".to_string(), Value::from(src.v_f64));
    map.insert("name".to_string(), Value::from(src.name.clone()));
    map.insert("vec".to_string(), Value::from(src.vec.clone()));
    map.insert("inner".to_string(), Value::from(inner.clone()));
    map.insert("terminator".to_string(), Value::from(terminator));

    let v = Value::from(map);
    let dest = data::unpack::<Dum>(v).unwrap();
    assert_eq!(src, dest);
}

#[derive(Debug, PartialEq, UnPack)]
struct Dum {
    v_u8: u8,
    v_u16: u16,
    v_u32: u32,
    v_u64: u64,
    v_f32: f32,
    v_f64: f64,
    name: String,
    vec: Vec<Value>,
    inner: Inner,
    terminator: Terminator,
}

#[derive(Debug, PartialEq, Default, UnPack)]
struct Inner {
    v_u8: u8,
}

#[derive(Debug, UnPack, PartialEq)]
enum Terminator {
    CRLF,
    CR,
    LF,
}

impl std::default::Default for Terminator {
    fn default() -> Self {
        Terminator::CRLF
    }
}

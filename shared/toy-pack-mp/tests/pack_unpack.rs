use std::collections::HashMap;
use toy_pack_derive::*;
use toy_pack_mp::{pack, unpack};

#[test]
fn same_struct_type() {
    let dum = Dum {
        v_u32: 123,
        v_f32: 1.5,
        v_f64: 1.5,
        name: "aiueo".to_owned(),
        borrowed_name: "kakikukeko",
    };
    let vec = pack(&dum).unwrap();
    let dest = unpack::<Dum>(vec.as_slice()).unwrap();

    assert_eq!(dum, dest);
}

#[test]
fn other_struct_type_drop_unknown_field() {
    let dum_p = DumPlusOne {
        v_u32: 123,
        v_f32: 1.5,
        v_f64: 1.5,
        name: "aiueo".to_owned(),
        borrowed_name: "kakikukeko",
        unknown_field: 9,
    };

    let vec = pack(&dum_p).unwrap();
    let dest = unpack::<Dum>(vec.as_slice()).unwrap();

    assert_eq!(dum_p.v_u32, dest.v_u32);
    assert_eq!(dum_p.v_f32, dest.v_f32);
    assert_eq!(dum_p.v_f64, dest.v_f64);
    assert_eq!(dum_p.name, dest.name);
    assert_eq!(dum_p.borrowed_name, dest.borrowed_name);
}

#[test]
fn unit_variant() {
    let vec = pack(&TestEnum::A).unwrap();
    let dest = unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(TestEnum::A, dest);
}

#[test]
fn newtype_variant() {
    let v = TestEnum::B(123);
    let vec = pack(&v).unwrap();
    let dest = unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(v, dest);
}

#[test]
fn tuple_variant() {
    let v = TestEnum::C(123, 456);
    let vec = pack(&v).unwrap();
    let dest = unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(v, dest);
}

#[test]
fn struct_variant() {
    let v = TestEnum::D {
        id: 1,
        name: "test".to_string(),
    };
    let vec = pack(&v).unwrap();
    let dest = unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(v, dest);
}

#[test]
fn ignore_if_none() {
    #[derive(Debug, PartialEq, Eq, Pack, Unpack)]
    #[toy(ignore_pack_if_none)]
    struct Test {
        a: Option<String>,
        b: bool,
    }

    let v = Test { a: None, b: true };

    let vec = pack(&v).unwrap();
    let dest = unpack::<Test>(vec.as_slice()).unwrap();

    assert_eq!(v, dest);
}

#[test]
fn ignore_if_none_all() {
    #[derive(Debug, Clone, PartialEq, Eq, Pack, Unpack)]
    #[toy(ignore_pack_if_none)]
    struct Test {
        a: Option<String>,
        b: Option<String>,
    }

    let v = Test { a: None, b: None };

    let vec = pack(&v).unwrap();
    let dest = unpack::<Test>(vec.as_slice()).unwrap();

    assert_eq!(v, dest);
}

#[test]
fn skip_none_struct() {
    #[derive(Debug, Clone, PartialEq, Pack, Unpack)]
    #[toy(ignore_pack_if_none)]
    struct Test {
        b: Option<HashMap<String, u32>>,
    }
    let b = {
        let mut src = HashMap::new();
        src.insert("a".to_string(), 10);
        Some(src)
    };
    let empty = Test { b: None };
    let some = Test { b };

    let src = vec![some.clone(), empty, some.clone()];
    let dest = pack(&src).unwrap();
    println!("{:?}", dest);
    let r = unpack::<Vec<Test>>(&dest).unwrap();

    assert_eq!(r, src);
}

//enum pattern ///////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Pack, Unpack)]
enum TestEnum {
    //unit variant
    A,
    //newtype variant
    B(u32),
    //tuple variant
    C(u32, u32),
    //struct variant
    D { id: u64, name: String },
}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::A
    }
}

//struct pattern ///////////////////////////////////////////////
#[derive(Debug, PartialEq, Pack, Unpack)]
struct Dum<'a> {
    v_u32: u32,
    v_f32: f32,
    v_f64: f64,
    name: String,
    borrowed_name: &'a str,
}

#[derive(Debug, PartialEq, Pack)]
struct DumPlusOne<'a> {
    v_u32: u32,
    v_f32: f32,
    v_f64: f64,
    name: String,
    borrowed_name: &'a str,
    unknown_field: u32,
}

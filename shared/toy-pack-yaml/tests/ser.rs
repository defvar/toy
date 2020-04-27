use std::collections::BTreeMap;
use toy_pack_derive::*;

#[test]
fn int() {
    let v = 256u32;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n256");
}

#[test]
fn float() {
    let v = 25.6;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n25.6");

    let v = std::f64::INFINITY;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n.inf");

    let v = std::f64::NEG_INFINITY;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n-.inf");
}

#[test]
fn vec() {
    let v = vec![1, 2, 3];
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
- 1
- 2
- 3"#;
    assert_eq!(r, expected);
}

#[test]
fn map() {
    let mut v = BTreeMap::new();
    v.insert(String::from("x"), 1);
    v.insert(String::from("y"), 2);
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
x: 1
y: 2"#;
    assert_eq!(r, expected);
}

#[test]
fn unit_variant() {
    #[derive(Pack, UnPack, PartialEq, Debug)]
    enum Variant {
        First,
        Second,
    }
    let v = Variant::First;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
First"#;
    assert_eq!(r, expected);
}

#[test]
fn newtype_variant() {
    #[derive(Pack, UnPack, PartialEq, Debug)]
    enum Variant {
        Size(usize),
    }
    let v = Variant::Size(127);
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
Size: 127"#;
    assert_eq!(r, expected);
}

#[test]
fn tuple_variant() {
    #[derive(Pack, UnPack, PartialEq, Debug)]
    enum Variant {
        One(u32),
        Two(u32, u32),
    }
    let v = Variant::Two(1, 2);
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
Two:
  - 1
  - 2"#;
    assert_eq!(r, expected);
}

#[test]
fn nested_struct() {
    #[derive(Pack, UnPack, PartialEq, Debug)]
    struct Outer {
        x: isize,
        y: String,
        z: bool,
        inner: Inner,
    }
    #[derive(Pack, UnPack, PartialEq, Debug, Default)]
    struct Inner {
        x: u32,
    }
    let v = Outer {
        x: -4,
        y: String::from("hi\tquoted"),
        z: true,
        inner: Inner { x: 123 },
    };
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
x: -4
y: "hi\tquoted"
z: true
inner:
  x: 123"#;
    assert_eq!(r, expected);
}

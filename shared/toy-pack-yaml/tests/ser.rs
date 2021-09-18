use serde::Serialize;
use std::collections::BTreeMap;

#[test]
fn int() {
    let v = 256u32;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n256\n");
}

#[test]
fn float() {
    let v = 25.6;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n25.6\n");

    let v = f64::INFINITY;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n.inf\n");

    let v = f64::NEG_INFINITY;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    assert_eq!(r, "---\n-.inf\n");
}

#[test]
fn vec() {
    let v = vec![1, 2, 3];
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
- 1
- 2
- 3
"#;
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
y: 2
"#;
    assert_eq!(r, expected);
}

#[test]
fn unit_variant() {
    #[derive(Serialize, PartialEq, Debug)]
    enum Variant {
        First,
    }
    let v = Variant::First;
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
First
"#;
    assert_eq!(r, expected);
}

#[test]
fn newtype_variant() {
    #[derive(Serialize, PartialEq, Debug)]
    enum Variant {
        Size(usize),
    }
    let v = Variant::Size(127);
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
Size: 127
"#;
    assert_eq!(r, expected);
}

#[test]
fn tuple_variant() {
    #[derive(Serialize, PartialEq, Debug)]
    enum Variant {
        Two(u32, u32),
    }
    let v = Variant::Two(1, 2);
    let r = toy_pack_yaml::pack_to_string(v).unwrap();
    let expected = r#"---
Two:
  - 1
  - 2
"#;
    assert_eq!(r, expected);
}

#[test]
fn nested_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct Outer {
        x: isize,
        y: String,
        z: bool,
        inner: Inner,
    }
    #[derive(Serialize, PartialEq, Debug, Default)]
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
  x: 123
"#;
    assert_eq!(r, expected);
}

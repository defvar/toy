use toy_pack_derive::*;
use toy_pack_yaml::deser;

#[test]
fn int() {
    let v = "256";
    let r = toy_pack_yaml::unpack::<u32>(v).unwrap();
    assert_eq!(r, 256u32);
}

#[test]
fn unit_variant() {
    #[derive(Pack, UnPack, PartialEq, Debug)]
    enum Variant {
        First,
        Second,
    }
    let v = r#"---
First"#;
    let r = toy_pack_yaml::unpack::<Variant>(v).unwrap();
    assert_eq!(r, Variant::First);
}

#[test]
fn nested_struct() {
    #[derive(Debug, UnPack, PartialEq, Default)]
    struct Outer {
        id: u32,
        name: String,
        age: Option<u32>,
        numbers: Vec<i64>,
        columns: Option<Vec<Inner>>,
    }

    #[derive(Debug, UnPack, Default, PartialEq)]
    struct Inner {
        name: String,
    }

    let s = "
id: 1
name: aiueo
age:
numbers:
  - 1
  - 2
  - 3
columns:
  - {name: 'a'}
  - {name: 'b'}
";

    let a = deser::unpack::<Outer>(s).unwrap();
    let expected = Outer {
        id: 1,
        name: "aiueo".to_owned(),
        age: None,
        numbers: vec![1, 2, 3],
        columns: Some(vec![
            Inner {
                name: "a".to_owned(),
            },
            Inner {
                name: "b".to_owned(),
            },
        ]),
    };
    assert_eq!(a, expected);
}

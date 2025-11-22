use serde::Deserialize;

#[test]
fn int() {
    let v = "256";
    let r = toy_pack_yaml::unpack::<u32>(v).unwrap();
    assert_eq!(r, 256u32);
}

#[test]
fn unit_variant() {
    #[derive(Deserialize, PartialEq, Debug)]
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
fn tuple_variant() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum Variant {
        One(u32),
        Two(u32, u32),
    }
    let v = "
!Two [1 ,2]
";
    let r = toy_pack_yaml::unpack::<Variant>(v).unwrap();
    assert_eq!(r, Variant::Two(1, 2));
}

#[test]
fn nested_struct() {
    #[derive(Debug, Deserialize, PartialEq, Default)]
    struct Outer {
        id: u32,
        name: String,
        age: Option<u32>,
        numbers: Vec<i64>,
        columns: Option<Vec<Inner>>,
    }

    #[derive(Debug, Deserialize, Default, PartialEq)]
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

    let a = toy_pack_yaml::unpack::<Outer>(s).unwrap();
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

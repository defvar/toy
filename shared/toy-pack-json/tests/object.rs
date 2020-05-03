use toy_pack_derive::*;
use toy_pack_json::{unpack, DecodeErrorKind};

#[test]
fn de_nested_struct() {
    #[derive(Debug, Unpack, PartialEq, Default)]
    struct Outer {
        id: u32,
        name: String,
        age: Option<u32>,
        numbers: Vec<i64>,
        columns: Option<Vec<Inner>>,
    }

    #[derive(Debug, Unpack, Default, PartialEq)]
    struct Inner {
        name: String,
    }

    let json = r#"
{
  "id": 1,
  "name": "aiueo",
  "numbers": [1, 2, 3],
  "columns": [{ "name": "a" }, { "name": "b" }]
}
"#;
    let r = unpack::<Outer>(json.as_bytes()).unwrap();
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
    assert_eq!(r, expected);
}

#[test]
fn de_struct_err_eof() {
    #[derive(Debug, Unpack, PartialEq, Default)]
    struct Data {
        id: u32,
    }

    let json = r#"
{
  "id": 1,
"#;
    match unpack::<Data>(json.as_bytes()) {
        Err(e) => match e.kind() {
            DecodeErrorKind::EofWhileParsingValue => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

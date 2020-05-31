use toy_pack_derive::*;
use toy_pack_json::{pack_to_string, unpack, DecodeErrorKind};
use toy_test_utils::unindent;

#[test]
fn ser_de_nested_struct() {
    #[derive(Debug, Pack, Unpack, PartialEq, Default)]
    #[toy(ignore_ser_if_none)]
    struct Outer {
        id: u32,
        name: String,
        age: Option<u32>,
        address: Option<String>,
        numbers: Vec<i64>,
        columns: Option<Vec<Inner>>,
    }

    #[derive(Debug, Pack, Unpack, Default, PartialEq)]
    struct Inner {
        name: String,
    }

    let json = r#"
{
  "id": 1,
  "name": "aiueo",
  "age": 1,
  "numbers": [1, 2, 3],
  "columns": [{ "name": "a" }, { "name": "b" }]
}
"#;
    let r = unpack::<Outer>(json.as_bytes()).unwrap();
    let expected = Outer {
        id: 1,
        name: "aiueo".to_owned(),
        age: Some(1),
        address: None,
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

    let r = pack_to_string(&expected).unwrap();
    assert_eq!(r, unindent(json));
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
        Err(e) if *e.kind() == DecodeErrorKind::EofWhileParsingValue => (),
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn de_struct_err_expected_comma() {
    #[derive(Debug, Unpack, PartialEq, Default)]
    struct Data {
        id: u32,
        name: String,
    }

    let json = r#"
{
  "id": 1
  "name": "aiueo"
}
"#;
    match unpack::<Data>(json.as_bytes()) {
        Err(e) if *e.kind() == DecodeErrorKind::ExpectedObjectCommaOrEnd { line: 4, column: 3 } => {
            ()
        }
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn de_struct_err_trailing_comma() {
    #[derive(Debug, Unpack, PartialEq, Default)]
    struct Data {
        id: u32,
    }

    let json = r#"
{
  "id": 1,
}
"#;
    match unpack::<Data>(json.as_bytes()) {
        Err(e) if *e.kind() == DecodeErrorKind::TrailingComma { line: 4, column: 1 } => (),
        other => panic!("unexpected result: {:?}", other),
    };
}

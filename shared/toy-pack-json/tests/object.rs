use serde::{Deserialize, Serialize};
use toy_pack_json::{pack_to_string, pack_to_string_pretty, unpack, DecodeError};
use toy_test_utils::unindent;

#[test]
fn ser_de_nested_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct Outer {
        id: u32,
        name: String,
        age: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<String>,
        numbers: Vec<i64>,
        columns: Option<Vec<Inner>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
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
fn ser_de_unit_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct Unit;
    let expected = Unit;
    let json = "null";
    let r = unpack::<Unit>(json.as_bytes()).unwrap();
    assert_eq!(r, expected);

    let r = pack_to_string(&expected).unwrap();
    assert_eq!(r, unindent(json));
}

#[test]
fn de_struct_err_eof() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct Data {
        id: u32,
    }

    let json = r#"
{
  "id": 1,
"#;
    match unpack::<Data>(json.as_bytes()) {
        Err(e) => match e {
            DecodeError::EofWhileParsingValue => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn de_struct_err_expected_comma() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
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
        Err(e) => match e {
            DecodeError::ExpectedObjectCommaOrEnd { line: 4, column: 3 } => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn de_struct_err_trailing_comma() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct Data {
        id: u32,
    }

    let json = r#"
{
  "id": 1,
}
"#;
    match unpack::<Data>(json.as_bytes()) {
        Err(e) => match e {
            DecodeError::TrailingComma { line: 4, column: 1 } => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn borrowed_value() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct Data<'a> {
        text: &'a str,
    }

    let json = r#"
{
  "text": "aiueo"
}
"#;

    let expected = Data { text: "aiueo" };

    let r = unpack::<Data>(json.as_bytes()).unwrap();
    assert_eq!(r, expected);
}

#[test]
fn pretty() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct Outer {
        id: u32,
        numbers: Vec<i64>,
        columns: Vec<Inner>,
    }

    #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
    struct Inner {
        name: String,
    }

    let d = Outer {
        id: 999,
        numbers: vec![11, 22, 33],
        columns: vec![
            Inner {
                name: "a".to_string(),
            },
            Inner {
                name: "b".to_string(),
            },
        ],
    };

    let json = pack_to_string_pretty(&d).unwrap();
    println!("{}", json);
}

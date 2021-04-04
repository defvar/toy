use std::collections::HashMap;
use toy_pack_json::{pack_to_string, unpack, DecodeError};

#[test]
fn de_map() {
    let expected = {
        let mut m = HashMap::new();
        m.insert("a".to_owned(), 1u32);
        m.insert("b".to_owned(), 2u32);
        m
    };
    let json = "{ \"a\": 1, \"b\": 2 }";
    let r = unpack::<HashMap<String, u32>>(json.as_bytes()).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn de_map_empty() {
    let expected = HashMap::new();
    let json = "{ }";
    let r = unpack::<HashMap<String, u32>>(json.as_bytes()).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn ser_map() {
    let d = {
        let mut m = HashMap::new();
        m.insert("a".to_owned(), 1u32);
        m
    };
    let expected = "{\"a\":1}";
    let r = pack_to_string(&d).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn ser_map_empty() {
    let d = HashMap::<String, String>::new();
    let expected = "{}";
    let r = pack_to_string(&d).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn de_map_err_eof() {
    let json = "{ \"a\": 1, \"b\": 2  ";
    match unpack::<HashMap<String, u32>>(json.as_bytes()) {
        Err(e) => match e {
            DecodeError::EofWhileParsingValue => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn de_map_err_trailing_comma() {
    let json = "{\"a\":1,}";
    match unpack::<HashMap<String, u32>>(json.as_bytes()) {
        Err(e) => match e {
            DecodeError::TrailingComma { line: 1, column: 8 } => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

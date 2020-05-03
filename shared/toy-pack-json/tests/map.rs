use std::collections::HashMap;
use toy_pack_json::{pack, unpack, DecodeErrorKind};

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
fn ser_map() {
    let d = {
        let mut m = HashMap::new();
        m.insert("a".to_owned(), 1u32);
        m
    };
    let expected = "{\"a\":1}";
    let r = pack(&d).unwrap();

    assert_eq!(std::str::from_utf8(r.as_slice()).unwrap(), expected);
}

#[test]
fn de_map_err_eof() {
    let json = "{ \"a\": 1, \"b\": 2  ";
    match unpack::<HashMap<String, u32>>(json.as_bytes()) {
        Err(e) => match e.kind() {
            DecodeErrorKind::EofWhileParsingValue => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

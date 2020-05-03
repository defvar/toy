use std::collections::HashMap;
use toy_pack_json::{unpack, DecodeErrorKind};

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

use toy_pack_json::{pack_to_string, unpack, DecodeError};

#[test]
fn de_seq() {
    let expected = vec![1u32, 2u32, 3u32];
    let json = "[1,2,3]";
    let r = unpack::<Vec<u32>>(json.as_bytes()).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn ser_seq() {
    let d = vec![1u32, 2u32, 3u32];
    let expected = "[1,2,3]";
    let r = pack_to_string(&d).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn de_seq_err_eof() {
    let json = "[1,2";
    match unpack::<Vec<u32>>(json.as_bytes()) {
        Err(e) => match e {
            DecodeError::EofWhileParsingValue => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

#[test]
fn de_seq_err_trailing_comma() {
    let json = "[1,2,3,]";
    match unpack::<Vec<u32>>(json.as_bytes()) {
        Err(e) => match e {
            DecodeError::TrailingComma { line: 1, column: 8 } => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

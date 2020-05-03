use toy_pack_json::{unpack, DecodeErrorKind};

#[test]
fn de_seq() {
    let expected = vec![1u32, 2u32, 3u32];
    let json = "[1,2,3]";
    let r = unpack::<Vec<u32>>(json.as_bytes()).unwrap();

    assert_eq!(r, expected);
}

#[test]
fn de_seq_err_eof() {
    let json = "[1,2";
    match unpack::<Vec<u32>>(json.as_bytes()) {
        Err(e) => match e.kind() {
            DecodeErrorKind::EofWhileParsingValue => (),
            other => panic!("unexpected result: {:?}", other),
        },
        other => panic!("unexpected result: {:?}", other),
    };
}

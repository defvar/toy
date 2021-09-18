use serde::{Deserialize, Serialize};
use toy_pack_json::{pack_to_string, unpack};
use toy_test_utils::unindent;

#[test]
fn ser_de_unit_variant() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Test {
        A,
        B,
    }
    let json = "\"B\"";

    let r = unpack::<Test>(json.as_bytes()).unwrap();
    assert_eq!(r, Test::B);

    let r = pack_to_string(&Test::B).unwrap();
    assert_eq!(r, json);
}

#[test]
fn ser_de_tuple_variant() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Test {
        A(u32, u32, u32),
        B(u8, u8),
    }
    let json = r#"{
      "B": [5, 6]
    }"#;

    let r = unpack::<Test>(json.as_bytes()).unwrap();
    assert_eq!(r, Test::B(5, 6));

    let r = pack_to_string(&Test::B(5, 6)).unwrap();
    assert_eq!(r, unindent(json));
}

#[test]
fn ser_de_new_type_variant() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Test {
        A(u32),
        B(u8),
    }
    let json = r#"{ "B": 5 }"#;

    let r = unpack::<Test>(json.as_bytes()).unwrap();
    assert_eq!(r, Test::B(5));

    let r = pack_to_string(&Test::B(5)).unwrap();
    assert_eq!(r, unindent(json));
}

#[test]
fn ser_de_struct_variant() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Test {
        A { id: u32, name: String },
    }
    let json = r#"{ "A": { "id": 5, "name": "aiueo" } }"#;
    let expected = Test::A {
        id: 5,
        name: "aiueo".to_string(),
    };

    let r = unpack::<Test>(json.as_bytes()).unwrap();
    assert_eq!(r, expected);

    let r = pack_to_string(&expected).unwrap();
    assert_eq!(r, unindent(json));
}

use toy_pack_derive::*;
use toy_pack_yaml::deser;

#[derive(Debug, UnPack, PartialEq, Default)]
struct Config {
    id: u32,
    name: String,
    age: Option<u32>,
    numbers: Vec<i64>,
    terminator: Terminator,
    columns: Option<Vec<Column>>,
}

#[derive(Debug, UnPack, Default, PartialEq)]
struct Column {
    name: String,
}

#[derive(Debug, UnPack, PartialEq)]
enum Terminator {
    CRLF,
    CR,
    LF,
}

impl std::default::Default for Terminator {
    fn default() -> Self {
        Terminator::CRLF
    }
}

#[test]
fn common_struct() {
    let s = "
id: 1
name: aiueo
age:
numbers:
  - 1
  - 2
  - 3
unknown_seq:
  - 1
  - 2
unknown_map:
  a: 1
  b: 2
terminator: LF
unknown_age: 1
columns:
  - {name: 'a'}
  - {name: 'b'}
";

    let a = deser::unpack::<Config>(s).unwrap();
    let expected = Config {
        id: 1,
        name: "aiueo".to_owned(),
        age: None,
        numbers: vec![1, 2, 3],
        columns: Some(vec![
            Column {
                name: "a".to_owned(),
            },
            Column {
                name: "b".to_owned(),
            },
        ]),
        terminator: Terminator::LF,
    };
    assert_eq!(a, expected);
}

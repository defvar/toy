use serde::{Deserialize, Serialize};

#[test]
fn de() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Param {
        hoge: String,
        moge: u32,
    }

    let query = "hoge=1&moge=2";
    let r = toy_pack_urlencoded::unpack::<Param>(query.as_bytes()).unwrap();
    assert_eq!(
        Param {
            hoge: "1".to_string(),
            moge: 2,
        },
        r
    );
}

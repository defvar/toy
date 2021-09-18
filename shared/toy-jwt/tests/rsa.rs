use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use toy_jwt::{Algorithm, Validation};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Calims {
    sub: String,
    company: String,
    exp: i64,
}

fn exp() -> i64 {
    let start = SystemTime::now();
    let iat = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    (iat + 3600) as i64
}

#[test]
fn safe_from_rsa_pem() {
    let key_pem = include_bytes!("test.co.jp-key.pem");
    let pub_pem = include_bytes!("test.co.jp-pub.pem");
    let c = Calims {
        sub: "sub".to_string(),
        company: "company".to_string(),
        exp: exp(),
    };
    let token = toy_jwt::encode::from_rsa_pem(&c, Algorithm::RS256, None, key_pem).unwrap();
    let r =
        toy_jwt::decode::from_rsa_pem::<Calims>(&token, Validation::new(Algorithm::RS256), pub_pem)
            .unwrap();

    assert_eq!(c, r)
}

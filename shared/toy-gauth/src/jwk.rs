use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct JWK {
    e: String,
    alg: String,
    kty: String,
    kid: String,
    n: String,
}

impl JWK {
    pub fn e(&self) -> &str {
        &self.e
    }

    pub fn kid(&self) -> &str {
        &self.kid
    }

    pub fn n(&self) -> &str {
        &self.n
    }
}

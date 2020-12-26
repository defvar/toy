use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,
    iat: u64,
    aud: String,
    iss: String,
    sub: String,
}

impl Claims {
    pub fn exp(&self) -> u64 {
        self.exp
    }

    pub fn iat(&self) -> u64 {
        self.iat
    }

    pub fn aud(&self) -> &str {
        &self.aud
    }

    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }
}

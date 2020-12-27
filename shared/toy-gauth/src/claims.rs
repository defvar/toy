use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) exp: u64,
    pub(crate) iat: u64,
    pub(crate) aud: String,
    pub(crate) iss: String,
    pub(crate) sub: String,
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

#[derive(Debug, Serialize)]
pub struct RequestTokenClaims {
    pub(crate) exp: u64,
    pub(crate) iat: u64,
    pub(crate) aud: String,
    pub(crate) iss: String,
    pub(crate) scope: String,
}

//! Model for authentication api.

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
}

impl Claims {
    pub fn new<P: Into<String>>(sub: P) -> Self {
        Self { sub: sub.into() }
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Secret {
    KeyPair(KeyPair),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    kid: String,
    private_key: String,
    public_key: String,
}

impl KeyPair {
    pub fn new(kid: String, private_key: String, public_key: String) -> Self {
        KeyPair {
            kid,
            private_key,
            public_key,
        }
    }

    pub fn kid(&self) -> &str {
        &self.kid
    }

    pub fn private_key(&self) -> &str {
        &self.private_key
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretList {
    items: Vec<Secret>,
    count: u32,
}

impl SecretList {
    pub fn new(items: Vec<Secret>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

use crate::constants;
use crate::error::GAuthError;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Deserialize)]
pub struct Credential {
    client_email: String,
    private_key: String,
}

impl Credential {
    pub fn from_key_file() -> Result<Credential, GAuthError> {
        let key_file_path =
            match std::env::var(constants::ENV_KEY_GOOGLE_API_CREDENTIALS).map_err(|_| {
                GAuthError::error(format!(
                    "not found key file. please set env {}.",
                    constants::ENV_KEY_GOOGLE_API_CREDENTIALS
                ))
            }) {
                Ok(id) => id,
                Err(e) => return Err(e),
            };

        let mut key_file = File::open(&key_file_path)?;

        let mut key_buf = Vec::new();
        let _ = key_file.read_to_end(&mut key_buf)?;
        let key = toy_pack_json::unpack::<Credential>(&key_buf)?;
        Ok(key)
    }

    pub fn client_email(&self) -> &str {
        &self.client_email
    }

    pub fn private_key(&self) -> &str {
        &self.private_key
    }
}

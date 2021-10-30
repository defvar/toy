use crate::algorithm::Algorithm;
use crate::error::JWTError;
use serde::Serialize;

pub fn from_rsa_pem<T: Serialize>(
    claims: &T,
    alg: Algorithm,
    kid: Option<String>,
    key: &[u8],
) -> Result<String, JWTError> {
    let mut header = jsonwebtoken::Header::new(alg.convert());
    header.kid = kid;
    let encoding_key =
        jsonwebtoken::EncodingKey::from_rsa_pem(key).map_err(|e| JWTError::error(e))?;
    let jws =
        jsonwebtoken::encode(&header, claims, &encoding_key).map_err(|e| JWTError::error(e))?;

    Ok(jws)
}

use crate::algorithm::Algorithm;
use crate::error::JWTError;
use std::collections::HashMap;
use toy_pack::ser::Serializable;

pub fn from_rsa_pem<T: Serializable>(
    claims: &T,
    alg: Algorithm,
    key: &[u8],
) -> Result<String, JWTError> {
    let json = toy_pack_json::pack_to_string(claims).map_err(|e| JWTError::error(e))?;
    let values: HashMap<String, serde_json::Value> =
        serde_json::from_str(&json).map_err(|e| JWTError::error(e))?;
    let header = jsonwebtoken::Header::new(alg.convert());
    let encoding_key =
        jsonwebtoken::EncodingKey::from_rsa_pem(key).map_err(|e| JWTError::error(e))?;
    let jws =
        jsonwebtoken::encode(&header, &values, &encoding_key).map_err(|e| JWTError::error(e))?;

    Ok(jws)
}

use crate::error::JWTError;
use crate::header::Header;
use crate::validation::Validation;
use crate::Algorithm;
use jsonwebtoken::DecodingKey;
use std::collections::HashMap;
use toy_pack::deser::DeserializableOwned;

pub fn from_rsa_pem<T: DeserializableOwned>(
    token: &str,
    v: Validation,
    key: &[u8],
) -> Result<T, JWTError> {
    let _ = header(token, &v)?;
    let key = DecodingKey::from_rsa_pem(key).map_err(|e| JWTError::error(e))?;
    claims(token, &key, v)
}

pub fn from_rsa_components<T: DeserializableOwned>(
    token: &str,
    v: Validation,
    modulus: &str,
    exponent: &str,
) -> Result<T, JWTError> {
    let _ = header(token, &v)?;
    let key = DecodingKey::from_rsa_components(modulus, exponent);
    claims(token, &key, v)
}

pub fn decode_header(token: &str) -> Result<Header, JWTError> {
    let h = jsonwebtoken::decode_header(token).map_err(|e| JWTError::error(e))?;
    Ok(Header {
        alg: h.alg.into(),
        kid: h.kid,
    })
}

fn header(token: &str, v: &Validation) -> Result<jsonwebtoken::Header, JWTError> {
    let h = jsonwebtoken::decode_header(token).map_err(|e| JWTError::error(e))?;
    match (&v.kid, &h.kid) {
        (Some(vk), Some(hk)) if vk != hk => {
            return Err(JWTError::error(format!("invalid kid. kid:{:?}", hk)))
        }
        (Some(_), None) => return Err(JWTError::error("unknwon kid.")),
        _ => (),
    };
    Ok(h)
}

fn claims<T: DeserializableOwned>(
    token: &str,
    key: &jsonwebtoken::DecodingKey,
    v: Validation,
) -> Result<T, JWTError> {
    let token_data =
        jsonwebtoken::decode::<HashMap<String, serde_json::Value>>(token, &key, &v.convert())
            .map_err(|e| JWTError::error(e))?;

    let json = serde_json::to_string(&token_data.claims).map_err(|e| JWTError::error(e))?;
    let r = toy_pack_json::unpack(json.as_bytes()).map_err(|e| JWTError::error(e))?;

    Ok(r)
}

use crate::error::JWTError;
use crate::header::Header;
use crate::validation::Validation;
use jsonwebtoken::DecodingKey;
use serde::de::DeserializeOwned;

pub fn from_rsa_pem<T: DeserializeOwned>(
    token: &str,
    v: Validation,
    key: &[u8],
) -> Result<T, JWTError> {
    let _ = header(token, &v)?;
    let key = DecodingKey::from_rsa_pem(key).map_err(|e| JWTError::error(e))?;
    claims(token, &key, v)
}

pub fn from_rsa_components<T: DeserializeOwned>(
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

fn claims<T: DeserializeOwned>(
    token: &str,
    key: &jsonwebtoken::DecodingKey,
    v: Validation,
) -> Result<T, JWTError> {
    let token_data =
        jsonwebtoken::decode::<T>(token, &key, &v.convert()).map_err(|e| JWTError::error(e))?;
    Ok(token_data.claims)
}

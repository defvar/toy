use crate::algorithm::Algorithm;
use crate::error::JWTError;
use serde::Serialize;

/// Create JWS from pem file.
///
/// If you are loading a RSA key from a .pem file.
/// This errors if the key is not a valid RSA key.
/// Encode the header and claims given and sign the payload using the algorithm from the header and the key.
/// Algorithm given is RSA. The key needs to be in the PEM format.
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

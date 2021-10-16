use crate::claims::RequestTokenClaims;
use crate::constants;
use crate::credential::Credential;
use crate::error::GAuthError;
use crate::scope::Scope;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use toy_h::{HttpClient, RequestBuilder, Response, Uri};

#[derive(Debug, Clone, Deserialize)]
pub struct GToken {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

impl GToken {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }

    pub fn token_type(&self) -> &str {
        &self.token_type
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GTokenError {
    error: String,
    error_description: String,
}

impl GTokenError {
    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn error_description(&self) -> &str {
        &self.error_description
    }
}

pub async fn request_token<T>(client: T, scope: Scope) -> Result<GToken, GAuthError>
where
    T: HttpClient,
{
    let key = Credential::from_key_file()?;

    let jws = create_jwt(key, scope)?;

    let body = format!(
        "grant_type={}&assertion={}",
        constants::TOKEN_REQUEST_GRANT_TYPE,
        jws
    );

    let res = client
        .post(Uri::from_static(constants::TOKEN_URL))
        .header(
            toy_h::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()
        .await?;

    if res.status().is_success() {
        let bytes = res.bytes().await?;
        toy_pack_json::unpack::<GToken>(&bytes).map_err(|e| e.into())
    } else {
        let bytes = res.bytes().await?;
        let err = toy_pack_json::unpack::<GTokenError>(&bytes)?;
        Err(GAuthError::request_token_error(err))
    }
}

fn create_jwt(c: Credential, scope: Scope) -> Result<String, GAuthError> {
    let start = SystemTime::now();
    let iat = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let claims = RequestTokenClaims {
        exp: iat + 3600,
        iat,
        aud: constants::TOKEN_URL.to_string(),
        iss: c.client_email().to_string(),
        scope: scope.uri().to_string(),
    };
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(&c.private_key().as_bytes())
        .map_err(|e| GAuthError::error(format!("jws creation error. {:?}.", e)))?;
    let jws = jsonwebtoken::encode(&header, &claims, &encoding_key)
        .map_err(|e| GAuthError::error(format!("jws creation error. {:?}.", e)))?;

    Ok(jws)
}

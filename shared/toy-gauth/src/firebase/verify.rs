use crate::claims::Claims;
use crate::constants;
use crate::error::GAuthError;
use crate::jwk::JWK;
use jsonwebtoken::DecodingKey;
use reqwest::{header, Url};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

thread_local! {
  static KEYS: RefCell<KeyCache> = RefCell::new(KeyCache::new());
}

#[derive(Debug, Deserialize)]
struct KeysResponse {
    keys: Vec<JWK>,
}

#[derive(Debug)]
struct KeyCache {
    max_age_secs: u64,
    keys: HashMap<String, JWK>,
}

impl KeyCache {
    pub fn new() -> KeyCache {
        Self {
            max_age_secs: 0,
            keys: HashMap::new(),
        }
    }
}

pub async fn verify_token(token: &str) -> Result<Claims, GAuthError> {
    let project_id = match std::env::var(constants::ENV_KEY_FIREBASE_ID).map_err(|_| {
        GAuthError::error(format!(
            "not found firebase config. please set env {}.",
            constants::ENV_KEY_FIREBASE_ID
        ))
    }) {
        Ok(id) => id,
        Err(e) => return Err(e),
    };

    let kid = match jsonwebtoken::decode_header(token).map(|header| header.kid) {
        Ok(Some(k)) => k,
        Ok(None) => return Err(GAuthError::authentication_failed("unknwon kid.")),
        Err(err) => return Err(GAuthError::authentication_failed(err)),
    };

    // validate: kid
    let jwk = match get_firebase_jwk(&kid).await {
        Ok(Some(k)) => k,
        Ok(None) => {
            return Err(GAuthError::authentication_failed("unknwon kid."));
        }
        Err(e) => {
            return Err(GAuthError::authentication_failed(e));
        }
    };

    let iss = match Url::parse(constants::FIREBASE_VALID_ISS_PREFIX)
        .unwrap()
        .join(&project_id)
        .map(|x| x.to_string())
    {
        Ok(iss) => iss,
        Err(e) => return Err(GAuthError::error(e)),
    };

    // validate: alg, iss
    let mut validation = jsonwebtoken::Validation {
        iss: Some(iss),
        ..jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256)
    };
    // validate: aud
    validation.set_audience(&[&project_id]);

    let key = DecodingKey::from_rsa_components(jwk.n(), jwk.e());
    let decoded_token = jsonwebtoken::decode::<Claims>(token, &key, &validation)
        .map_err(|e| GAuthError::authentication_failed(e));

    decoded_token.map(|x| x.claims)
}

async fn get_firebase_jwk(kid: &str) -> Result<Option<JWK>, reqwest::Error> {
    let max_age_secs = KEYS.with(|kc| kc.borrow().max_age_secs);
    let start = SystemTime::now();
    let now = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let limit = start
        .checked_add(Duration::from_secs(max_age_secs))
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    if limit > now {
        tracing::debug!("search jwk from cache.");
        return Ok(KEYS.with(|kc| kc.borrow().keys.get(kid).map(|x| x.clone())));
    }

    tracing::info!("reflesh jwk list.");
    let resp = reqwest::get(constants::JWK_URL).await?;

    let max_age_secs = match resp.headers().get(header::CACHE_CONTROL) {
        Some(v) => v
            .to_str()
            .map(|x| parse_cache_control(x))
            .unwrap_or_else(|_| 0),
        None => 0,
    };
    let body = resp.json::<KeysResponse>().await?;

    let mut key_map = HashMap::new();
    for key in body.keys {
        key_map.insert(key.kid().to_string(), key);
    }

    let r = KEYS.with(|kc| {
        let mut kc = kc.borrow_mut();
        kc.keys = key_map;
        kc.max_age_secs = max_age_secs;
        kc.keys.get(kid).map(|x| x.clone())
    });

    Ok(r)
}

fn parse_cache_control(v: &str) -> u64 {
    v.split(",")
        .map(|x| x.trim().to_lowercase())
        .filter(|x| x.starts_with("max-age="))
        .map(|x| x.replace("max-age=", ""))
        .last()
        .map(|x| x.parse::<u64>().unwrap_or_else(|_| 0))
        .unwrap_or_else(|| 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_cache_control() {
        let h = "public, max-age=24845, must-revalidate, no-transform";
        assert_eq!(parse_cache_control(h), 24845);

        let h = "public, max-age=24845, mAx-aGe=1,";
        assert_eq!(parse_cache_control(h), 1);

        let h = "xxxxxxxxxxxxxxx";
        assert_eq!(parse_cache_control(h), 0);
    }
}

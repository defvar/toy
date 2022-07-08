//! # Auth Implementation JWT.

use toy_api::authentication::Claims;
use toy_api_server::authentication::{Auth, AuthUser};
use toy_api_server::toy_h::HttpClient;
use toy_api_server::ApiError;
use toy_jwt::{Algorithm, Validation};

#[derive(Debug, Clone)]
pub struct JWTAuth;

impl JWTAuth {
    pub fn new() -> Self {
        JWTAuth
    }
}

impl<T> Auth<T> for JWTAuth
where
    T: HttpClient,
{
    type F = std::future::Ready<Result<AuthUser, ApiError>>;

    fn verify(&self, _client: T, token: String) -> Self::F {
        let h =
            toy_jwt::decode::decode_header(&token).map_err(|e| ApiError::authentication_failed(e));

        let h = match h {
            Ok(v) => v,
            Err(e) => return std::future::ready(Err(e)),
        };

        if h.kid.is_none() {
            return std::future::ready(Err(ApiError::authentication_failed("unknown kid")));
        }

        // get from cache
        let key_pair = toy_api_server::context::server::key_pair(&h.kid.unwrap());

        let key_pair = match key_pair {
            Some(v) => v,
            None => return std::future::ready(Err(ApiError::authentication_failed("unknown kid"))),
        };

        let pub_key = key_pair.public_key();
        let validation = Validation::new(Algorithm::RS256).exp(false);
        let claims =
            toy_jwt::decode::from_rsa_pem::<Claims>(&token, validation, pub_key.as_bytes())
                .map_err(|e| ApiError::authentication_failed(e));

        let claims = match claims {
            Ok(v) => v,
            Err(e) => return std::future::ready(Err(e)),
        };

        std::future::ready(Ok(AuthUser::new(claims.sub())))
    }
}

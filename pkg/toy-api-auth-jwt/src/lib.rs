//! # Auth Implementation for Service Account(JWT).

use std::future::Future;
use std::marker::PhantomData;
use toy_api::authentication::Claims;
use toy_api_server::authentication::{Auth, AuthUser};
use toy_api_server::toy_h::HttpClient;
use toy_api_server::ApiError;
use toy_jwt::{Algorithm, Validation};

#[derive(Debug, Clone)]
pub struct ServiceAccountAuth {}

impl ServiceAccountAuth {
    pub fn new() -> Result<Self, ApiError> {
        Ok(ServiceAccountAuth {})
    }
}

impl<T> Auth<T> for ServiceAccountAuth
where
    T: HttpClient,
{
    type F = std::future::Ready<Result<AuthUser, ApiError>>;

    fn verify(&self, _client: T, token: String) -> Self::F {
        // let h = toy_jwt::decode::decode_header(&token)
        //     .map_err(|e| ApiError::authentication_failed(e))?;
        //
        // // gey pub key from h.kid.
        //
        // let key = &[0u8];
        // let claims: Claims =
        //     toy_jwt::decode::from_rsa_pem(&token, Validation::new(Algorithm::RS256), key)
        //         .map_err(|e| ApiError::authentication_failed(e))?;

        // std::future::ready(Ok(AuthUser::new(claims.sub())))
        std::future::ready(Ok(AuthUser::new("wip")))
    }
}

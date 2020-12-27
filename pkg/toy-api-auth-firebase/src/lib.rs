#![feature(type_alias_impl_trait)]

//! # Auth Implementation for Firebase.

use std::future::Future;
use toy_api_server::auth::{Auth, AuthUser};
use toy_api_server::reqwest;
use toy_api_server::ApiError;

#[derive(Debug, Clone)]
pub struct FireAuth;

impl FireAuth {
    pub fn new() -> Result<Self, ApiError> {
        Ok(Self)
    }
}

impl Auth for FireAuth {
    type F = impl Future<Output = Result<AuthUser, crate::ApiError>> + Send;

    fn verify(&self, client: reqwest::Client, token: String) -> Self::F {
        async move {
            let claims = toy_gauth::firebase::verify_token(client, &token)
                .await
                .map_err(|e| ApiError::authentication_failed(e))?;
            Ok(AuthUser::new(claims.sub()))
        }
    }
}

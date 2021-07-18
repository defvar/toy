#![feature(min_type_alias_impl_trait)]

//! # Auth Implementation for Firebase.

use std::future::Future;
use toy_api_server::authentication::{Auth, AuthUser};
use toy_api_server::toy_h::HttpClient;
use toy_api_server::ApiError;

#[derive(Debug, Clone)]
pub struct FireAuth;

impl FireAuth {
    pub fn new() -> Self {
        FireAuth
    }
}

impl<T> Auth<T> for FireAuth
where
    T: HttpClient,
{
    type F = impl Future<Output = Result<AuthUser, crate::ApiError>> + Send;

    fn verify(&self, client: T, token: String) -> Self::F {
        async move {
            let claims = toy_gauth::firebase::verify_token::<T>(client, &token)
                .await
                .map_err(|e| ApiError::authentication_failed(e))?;
            Ok(AuthUser::new(claims.sub()))
        }
    }
}

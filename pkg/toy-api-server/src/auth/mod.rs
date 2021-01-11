//! Authorization.
//!

mod filters;

use crate::ApiError;
pub use filters::auth_filter;
use reqwest::Client;

/// Authenticated User Infomation.
#[derive(Clone)]
pub struct AuthUser {
    uid: String,
}

impl AuthUser {
    pub fn new<T: Into<String>>(uid: T) -> AuthUser {
        Self { uid: uid.into() }
    }
}

impl warp::Reply for AuthUser {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::Response::new(format!("{}", self.uid).into())
    }
}

/// Authorization Operation.
pub trait Auth: Clone + Send + Sync {
    type F: std::future::Future<Output = Result<AuthUser, ApiError>> + Send;

    /// verify string token.
    /// token is 'Authorization: Bearer {token}' of Http Header.
    fn verify(&self, client: reqwest::Client, token: String) -> Self::F;
}

/// Implementation No Auth.
#[derive(Clone)]
pub struct NoAuth;

impl Auth for NoAuth {
    type F = std::future::Ready<Result<AuthUser, ApiError>>;

    fn verify(&self, _client: Client, _token: String) -> Self::F {
        std::future::ready(Ok(AuthUser::new("unknown")))
    }
}

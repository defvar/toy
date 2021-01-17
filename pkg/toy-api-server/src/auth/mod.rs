//! Authorization.
//!

mod filters;

use crate::ApiError;
pub use filters::auth_filter;
use std::marker::PhantomData;
use toy_h::HttpClient;

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
pub trait Auth<T>: Clone + Send + Sync {
    type F: std::future::Future<Output = Result<AuthUser, ApiError>> + Send;

    /// verify string token.
    /// token is 'Authorization: Bearer {token}' of Http Header.
    fn verify(&self, client: T, token: String) -> Self::F;
}

/// Implementation No Auth.
#[derive(Clone)]
pub struct NoAuth<T> {
    _t: PhantomData<T>,
}

impl<T> NoAuth<T> {
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }
}

impl<T> Auth<T> for NoAuth<T>
where
    T: HttpClient,
{
    type F = std::future::Ready<Result<AuthUser, ApiError>>;

    fn verify(&self, _client: T, _token: String) -> Self::F {
        std::future::ready(Ok(AuthUser::new("unknown")))
    }
}

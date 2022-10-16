//! Authentication.

mod authenticator;

use crate::ApiError;
pub use authenticator::authenticate;
use std::fmt;
use std::future::Future;
use toy_h::HttpClient;

/// Authenticated User Infomation.
#[derive(Clone)]
pub struct AuthUser {
    name: String,
}

impl AuthUser {
    pub fn new<T: Into<String>>(name: T) -> AuthUser {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Debug for AuthUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthUser")
            .field("name", &self.name)
            .finish()
    }
}

/// Authentication Operation.
pub trait Auth<T>: Clone + Send + Sync {
    type F: std::future::Future<Output = Result<AuthUser, ApiError>> + Send;

    /// verify string token.
    /// token is 'Authorization: Bearer {token}' of Http Header.
    fn verify(&self, client: T, token: String) -> Self::F;
}

/// Implementation Auth Operations for User or ServiceAccount.
#[derive(Clone)]
pub struct CommonAuths<T, U> {
    user: T,
    service_account: U,
}

impl<T, U> CommonAuths<T, U> {
    pub fn new(user: T, service_account: U) -> Self {
        Self {
            user,
            service_account,
        }
    }
}

impl<T, U, H> Auth<H> for CommonAuths<T, U>
where
    T: Auth<H>,
    U: Auth<H>,
    H: HttpClient,
{
    type F = impl Future<Output = Result<AuthUser, crate::ApiError>> + Send;

    fn verify(&self, client: H, token: String) -> Self::F {
        let u = self.user.clone();
        let s = self.service_account.clone();
        async move {
            tracing::debug!("first authentication, user.");
            let r = u.verify(client.clone(), token.clone()).await;
            match r {
                Ok(u) => Ok(u),
                Err(_) => {
                    tracing::debug!("next authentication, service account.");
                    s.verify(client.clone(), token.clone()).await
                }
            }
        }
    }
}

/// Implementation No Auth.
#[derive(Clone)]
pub struct NoAuth;

impl NoAuth {
    pub fn new() -> Self {
        Self
    }
}

impl<T> Auth<T> for NoAuth
where
    T: HttpClient,
{
    type F = std::future::Ready<Result<AuthUser, ApiError>>;

    fn verify(&self, _client: T, _token: String) -> Self::F {
        std::future::ready(Ok(AuthUser::new("unknown")))
    }
}

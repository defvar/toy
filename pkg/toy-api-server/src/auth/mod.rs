mod filters;

pub use filters::auth_filter;

/// Authenticated User Infomation.
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

/// Authlization Operation.
pub trait Auth: Send + Sync {
    type F: std::future::Future<Output = Result<AuthUser, crate::ApiError>> + Send;

    fn verify(&self, client: reqwest::Client, token: String) -> Self::F;
}

use crate::auth::{Auth, AuthUser};
use crate::ApiError;
use warp::{Filter, Rejection};

pub fn auth_filter(
    auth: impl Auth + Clone,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::any()
        .map(move || auth.clone())
        .and(warp::header::header::<String>("Authorization"))
        .and_then(|a, b| handle_auth(a, b))
}

async fn handle_auth(auth: impl Auth + Clone, authen_str: String) -> Result<AuthUser, Rejection> {
    if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
        let token = authen_str[6..authen_str.len()].trim();
        let user = auth.verify(token.to_string()).await;
        match user {
            Ok(u) => Ok(u),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(ApiError::authentication_failed(
            "not found Bearer token.",
        )))
    }
}

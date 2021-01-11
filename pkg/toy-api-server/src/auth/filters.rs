use crate::auth::{Auth, AuthUser};
use crate::ApiError;
use warp::{Filter, Rejection};

/// warp filter for auth.
pub fn auth_filter(
    auth: impl Auth + Clone,
    client: reqwest::Client,
) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    auth_bearer(auth, client).or_else(|e| auth_dev(e))
}

fn auth_bearer(
    auth: impl Auth + Clone,
    client: reqwest::Client,
) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    warp::any()
        .map(move || (auth.clone(), client.clone()))
        .and(warp::header::header::<String>("Authorization"))
        .and_then(|(a, b), c| handle_auth(a, b, c))
}

async fn auth_dev(e: Rejection) -> Result<(AuthUser,), Rejection> {
    if let Ok(v) = std::env::var("DEV_AUTH") {
        if v == "none" {
            return Ok((AuthUser::new("dev".to_string()),));
        }
    }
    Err(e)
}

async fn handle_auth(
    auth: impl Auth + Clone,
    client: reqwest::Client,
    authen_str: String,
) -> Result<AuthUser, Rejection> {
    if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
        let token = authen_str[6..authen_str.len()].trim();
        let user = auth.verify(client, token.to_string()).await;
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

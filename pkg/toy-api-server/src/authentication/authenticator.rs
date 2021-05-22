use crate::authentication::{Auth, AuthUser};
use crate::authorization::authorize;
use crate::context::Context;
use crate::ApiError;
use toy_h::HttpClient;
use warp::http::Method;
use warp::path::FullPath;
use warp::{reject, Filter, Rejection};

/// warp filter for authentication.
pub fn authenticate<T>(
    auth: impl Auth<T> + Clone,
    client: T,
) -> impl Filter<Extract = (Context,), Error = Rejection> + Clone
where
    T: HttpClient,
{
    auth_bearer(auth, client)
        .or_else(|e| auth_dev(e))
        .and(warp::path::full())
        .and(warp::method())
        .and_then(|user, path: FullPath, method: Method| async move {
            let ctx = Context::new(user, path.as_str(), method.as_str());
            match authorize(&ctx, Vec::new()) {
                Ok(_) => Ok(ctx),
                Err(e) => {
                    tracing::info!("forbidden: {}", e.error_message());
                    Err(reject::custom(e))
                }
            }
        })
}

fn auth_bearer<T>(
    auth: impl Auth<T> + Clone,
    client: T,
) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone
where
    T: HttpClient,
{
    warp::any()
        .map(move || (auth.clone(), client.clone()))
        .and(warp::header::header::<String>("Authorization"))
        .and_then(|(a, b), c| handle_auth(a, b, c))
}

async fn auth_dev(e: Rejection) -> Result<(AuthUser,), Rejection> {
    if let Ok(v) = std::env::var("TOY_AUTHENTICATION") {
        if v == "none" {
            tracing::warn!("skip authentication.");
            return Ok((AuthUser::new("dev".to_string()),));
        }
    }
    Err(e)
}

async fn handle_auth<T>(
    auth: impl Auth<T> + Clone,
    client: T,
    authen_str: String,
) -> Result<AuthUser, Rejection>
where
    T: HttpClient,
{
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

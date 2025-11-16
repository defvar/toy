use crate::authentication::{Auth, AuthUser};
use crate::context::{Context, ServerState};
use crate::ApiError;
use toy_api_http_common::axum::http::request::Parts;
use toy_api_http_common::axum::RequestPartsExt;
use toy_api_http_common::headers::authorization::Bearer;
use toy_api_http_common::headers::Authorization;
use toy_api_http_common::typed_header::TypedHeader;

pub async fn authenticate<S>(parts: &mut Parts, state: &S) -> Result<Context, ApiError>
where
    S: ServerState,
{
    if let Ok(v) = std::env::var("TOY_AUTHENTICATION") {
        if v == "none" {
            tracing::warn!("skip authentication.");
            return Ok(Context::new(
                AuthUser::new("dev".to_string()),
                parts.uri.path(),
                parts.method.as_str(),
            ));
        }
    }
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| ApiError::authentication_failed("not found Bearer token."))?;

    let user = state
        .auth()
        .verify(state.client().clone(), bearer.token().to_string())
        .await;
    match user {
        Ok(u) => {
            return Ok(Context::new(u, parts.uri.path(), parts.method.as_str()));
        }
        Err(e) => Err(ApiError::authentication_failed(e)),
    }
}

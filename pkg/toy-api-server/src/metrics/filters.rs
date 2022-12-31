use crate::context::{Context, ServerState, WrappedState};
use crate::metrics::handlers;
use crate::ApiError;
use toy_api::common::PostOption;
use toy_api_http_common::axum::extract::{Query, State};
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;

pub async fn post<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Query(api_opt): Query<PostOption>,
    request: Bytes,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    handlers::post(ctx, api_opt, request, state.raw().metrics_store()).await
}

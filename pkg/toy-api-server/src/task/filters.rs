use crate::context::{Context, ServerState, WrappedState};
use crate::task::handlers;
use crate::ApiError;
use toy_api::task::{LogOption, PostOption, TaskListOption};
use toy_api_http_common::axum::extract::{Path, Query, State};
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
    handlers::post(ctx, api_opt, request, state.raw().task_store()).await
}

pub async fn list<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Query(api_opt): Query<TaskListOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    handlers::list(ctx, api_opt, state.raw().task_log_store()).await
}

pub async fn log<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Path(key): Path<String>,
    Query(api_opt): Query<LogOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    handlers::log(ctx, key, api_opt, state.raw().task_log_store()).await
}

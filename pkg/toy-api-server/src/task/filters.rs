use crate::context::{Context, ServerState, WrappedState};
use crate::store::kv;
use crate::task::handlers;
use crate::{common, ApiError};
use toy_api::common::{FindOption, PostOption};
use toy_api::task::{LogOption, PendingTask, TaskListOption};
use toy_api_http_common::axum::extract::{Path, Query, State};
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;

pub async fn find<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Path(key): Path<String>,
    Query(api_opt): Query<FindOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    common::handler::find2(
        ctx,
        state.raw().kv_store(),
        common::constants::generate_key(common::constants::PENDINGS_KEY_PREFIX, key),
        api_opt,
        kv::FindOption::new(),
        |v: PendingTask| v,
    )
    .await
}

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

pub async fn finish<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Path(key): Path<String>,
    Query(api_opt): Query<PostOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    handlers::finish(ctx, state.raw().kv_store(), key, api_opt).await
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

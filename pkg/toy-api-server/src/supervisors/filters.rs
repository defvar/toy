use crate::common::validator::OkValidator;
use crate::context::{Context, ServerState, WrappedState};
use crate::store::kv;
use crate::supervisors::handlers;
use crate::{common, ApiError};
use toy_api::common::{DeleteOption, FindOption, PutOption};
use toy_api::supervisors::{Supervisor, SupervisorList, SupervisorListOption};
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
        common::constants::generate_key(common::constants::SUPERVISORS_KEY_PREFIX, key),
        api_opt,
        kv::FindOption::new(),
        |v: Supervisor| v,
    )
    .await
}

pub async fn list<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Query(api_opt): Query<SupervisorListOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    common::handler::list2(
        ctx,
        state.raw().kv_store(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        api_opt,
        |_: &SupervisorListOption| kv::ListOption::new(),
        |v: Vec<Supervisor>| SupervisorList::new(v),
    )
    .await
}

pub async fn put<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Path(key): Path<String>,
    Query(api_opt): Query<PutOption>,
    request: Bytes,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    common::handler::put2(
        ctx,
        state.raw().kv_store(),
        common::constants::SUPERVISORS_KEY_PREFIX,
        key,
        api_opt,
        kv::PutOption::new(),
        request,
        OkValidator::<Supervisor>::new(),
    )
    .await
}

pub async fn delete<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Path(key): Path<String>,
    Query(api_opt): Query<DeleteOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    common::handler::delete2(
        ctx,
        state.raw().kv_store(),
        common::constants::generate_key(common::constants::SUPERVISORS_KEY_PREFIX, key),
        api_opt,
        kv::DeleteOption::new(),
    )
    .await
}

pub async fn beat<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    handlers::beat(ctx, state.raw().kv_store(), key, None).await
}

use crate::context::{Context, ServerState, WrappedState};
use crate::rbac::validator::RoleValidator;
use crate::store::kv;
use crate::{common, ApiError};
use toy_api::common::{DeleteOption, FindOption, PutOption};
use toy_api::role::{Role, RoleList, RoleListOption};
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
        common::constants::generate_key(common::constants::ROLE_KEY_PREFIX, key),
        api_opt,
        kv::FindOption::new(),
        |v: Role| v,
    )
    .await
}

pub async fn list<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Query(api_opt): Query<RoleListOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    common::handler::list2(
        ctx,
        state.raw().kv_store(),
        common::constants::ROLE_KEY_PREFIX,
        api_opt,
        |_: &RoleListOption| kv::ListOption::new(),
        |v: Vec<Role>| RoleList::new(v),
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
        common::constants::ROLE_KEY_PREFIX,
        key,
        api_opt,
        kv::PutOption::new(),
        request,
        RoleValidator,
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
        common::constants::generate_key(common::constants::ROLE_KEY_PREFIX, key),
        api_opt,
        kv::DeleteOption::new(),
    )
    .await
}

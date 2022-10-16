use crate::context::{Context, ServerState, WrappedState};
use crate::graph::validator::GraphPutValidator;
use crate::store::kv;
use crate::store::kv::ListOption;
use crate::{common, ApiError};
use toy_api::common::{DeleteOption, FindOption, PutOption};
use toy_api::graph::{Graph, GraphList, GraphListOption};
use toy_api::supervisors::Supervisor;
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
        common::constants::generate_key(common::constants::GRAPHS_KEY_PREFIX, key),
        api_opt,
        kv::FindOption::new(),
        |v: Supervisor| v,
    )
    .await
}

pub async fn list<S>(
    ctx: Context,
    State(state): State<WrappedState<S>>,
    Query(api_opt): Query<GraphListOption>,
) -> Result<impl IntoResponse, ApiError>
where
    S: ServerState,
{
    common::handler::list2(
        ctx,
        state.raw().kv_store(),
        common::constants::GRAPHS_KEY_PREFIX,
        api_opt,
        |_: &GraphListOption| ListOption::new(),
        |v: Vec<Graph>| GraphList::new(v),
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
        common::constants::GRAPHS_KEY_PREFIX,
        key,
        api_opt,
        kv::PutOption::new(),
        request,
        GraphPutValidator,
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
        common::constants::generate_key(common::constants::GRAPHS_KEY_PREFIX, key),
        api_opt,
        kv::DeleteOption::new(),
    )
    .await
}

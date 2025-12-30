use crate::common::constants;
use crate::context::Context;
use crate::store::kv;
use crate::store::kv::{Find, KvStore, Put, PutOption, PutResult};
use crate::ApiError;
use toy_api::common::{self as api_common};
use toy_api::graph::Graph;
use toy_api::task::{PendingResult, PendingTask};
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::reply;
use toy_core::prelude::TaskId;
use toy_h::HttpClient;

pub async fn dispatch<T>(
    ctx: Context,
    store: &impl KvStore<T>,
    graph_name: String,
    opt: api_common::PostOption,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let graph = match store
        .ops()
        .find::<Graph>(
            store.con().unwrap(),
            constants::generate_key(constants::GRAPHS_KEY_PREFIX, &graph_name),
            kv::FindOption::new(),
        )
        .await
    {
        Ok(v) => match v {
            Some(v) => v.into_value(),
            None => return Err(ApiError::not_found(&graph_name)),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            return Err(ApiError::error(e));
        }
    };

    let id = TaskId::new();
    let pending = PendingTask::new(id, graph);
    let key = constants::pending_key(id);
    match store
        .ops()
        .put(
            store.con().unwrap(),
            key,
            pending,
            PutOption::new().with_create_only(),
        )
        .await
    {
        Ok(PutResult::Create) => Ok(reply::into_response(
            &(PendingResult::from_id(id)),
            opt.format(),
            opt.indent(),
        )),
        Ok(PutResult::Update(_)) => unreachable!(),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

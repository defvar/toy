use crate::http::Status;
use crate::supervisor::SupervisorContext;
use crate::{Request, RunTaskResponse, SupervisorError};
use toy_api::common::{ListOptionLike, PostOption};
use toy_api::services::{ServiceSpec, ServiceSpecListOption};
use toy_api::task::{AllocateResponse, PendingTask};
use toy_api_client::ApiClient;
use toy_api_http_common::axum::extract::{Query, State};
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;

pub async fn index() -> impl IntoResponse {
    "Hello"
}

pub async fn status<C>(
    State(ctx): State<SupervisorContext<C>>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let st = Status {
        name: ctx.name().to_owned(),
        started_at: ctx.started_at_str(),
        running_tasks: ctx.tasks().await,
    };
    match toy_pack_json::pack_to_string(&st) {
        Ok(v) => Ok(v),
        Err(e) => {
            tracing::error!(err = %e);
            Err(SupervisorError::error(""))
        }
    }
}

pub async fn services<C>(
    State(ctx): State<SupervisorContext<C>>,
    Query(opt): Query<ServiceSpecListOption>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let specs = ctx
        .schemas()
        .iter()
        .map(|x| {
            ServiceSpec::new(
                x.service_type().clone(),
                x.port_type().clone(),
                x.schema().cloned(),
            )
        })
        .collect::<Vec<_>>();
    let format = opt.common().format();
    let indent = opt.common().indent();

    Ok(toy_api_http_common::reply::into_response(
        &specs, format, indent,
    ))
}

pub async fn tasks<C>(
    State(mut ctx): State<SupervisorContext<C>>,
    Query(opt): Query<PostOption>,
    request: Bytes,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let format = opt.format();
    let pending = toy_api_http_common::codec::decode::<_, PendingTask>(request, format)?;

    let v = toy_core::data::pack(pending.graph())
        .map_err(|e| Into::<toy_api_http_common::Error>::into(e))?;
    let g = Graph::from(v).map_err(|e| Into::<SupervisorError>::into(e))?;
    tracing::debug!("{:?}", g);
    let (o_tx, _) = toy_core::oneshot::channel::<RunTaskResponse, ServiceError>();
    let req = Request::RunTask(pending.task_id(), g, o_tx);
    let _ = ctx.tx_mut().send_ok(req).await;
    Ok(toy_api_http_common::reply::into_response(
        &AllocateResponse::ok(pending.task_id()),
        format,
        None,
    ))
}

pub async fn shutdown<C>(
    State(mut ctx): State<SupervisorContext<C>>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    match ctx.tx_mut().send_ok(Request::Shutdown).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!(err = %e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

use crate::context::SupervisorContext;
use crate::exporters::supervisor_metrics;
use crate::http::Status;
use crate::{Request, RunTaskResponse, SupervisorError, TaskResponse};
use chrono::Utc;
use toy_api::common::{FindOption, ListOption, ListOptionLike, PostOption};
use toy_api::services::{ServiceSpec, ServiceSpecListOption};
use toy_api::task::{AllocateResponse, PendingTask};
use toy_api_client::ApiClient;
use toy_api_http_common::axum::extract::{Path, Query, State};
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;
use toy_core::graph::Graph;
use toy_core::metrics;
use toy_core::task::TaskId;

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
        running_tasks: ctx.task_id_and_graph_name().await,
        last_task_executed_at: ctx.last_task_executed_at().await,
        last_event_exported_at: ctx.last_event_exported_at().await,
        last_metrics_exported_at: ctx.last_metrics_exported_at().await,
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

pub async fn tasks_post<C>(
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
    let (o_tx, _) = toy_core::oneshot::channel::<RunTaskResponse>();
    let req = Request::RunTask(pending.task_id(), g, o_tx);
    let _ = ctx.tx_mut().send_ok(req).await;
    Ok(toy_api_http_common::reply::into_response(
        &AllocateResponse::ok(pending.task_id()),
        format,
        None,
    ))
}

pub async fn tasks_find<C>(
    State(mut ctx): State<SupervisorContext<C>>,
    Path(key): Path<String>,
    Query(opt): Query<FindOption>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    match TaskId::parse_str(&key) {
        Ok(id) => {
            let (tx, rx) = toy_core::oneshot::channel::<Option<TaskResponse>>();
            let req = Request::Task(id, tx);
            let _ = ctx.tx_mut().send_ok(req).await;
            match rx.recv().await {
                Some(Some(t)) => Ok(toy_api_http_common::reply::into_response(
                    &t,
                    opt.format(),
                    opt.indent(),
                )),
                _ => Err(SupervisorError::not_found(&key)),
            }
        }
        Err(_) => Err(SupervisorError::task_id_invalid_format(key)),
    }
}

pub async fn tasks_list<C>(
    State(mut ctx): State<SupervisorContext<C>>,
    Query(opt): Query<ListOption>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let (tx, rx) = toy_core::oneshot::channel::<Vec<TaskResponse>>();
    let req = Request::Tasks(tx);
    let _ = ctx.tx_mut().send_ok(req).await;
    match rx.recv().await {
        Some(tasks) => Ok(toy_api_http_common::reply::into_response(
            &tasks,
            opt.format(),
            opt.indent(),
        )),
        _ => Ok(toy_api_http_common::reply::into_response(
            &Vec::<TaskResponse>::new(),
            opt.format(),
            opt.indent(),
        )),
    }
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

pub async fn event_buffers<C>(
    State(_ctx): State<SupervisorContext<C>>,
    Query(opt): Query<PostOption>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let format = opt.format();
    let events = metrics::context::events().records().await;
    Ok(toy_api_http_common::reply::into_response(
        &events,
        format,
        opt.indent(),
    ))
}

pub async fn metrics<C>(
    State(ctx): State<SupervisorContext<C>>,
    Query(opt): Query<PostOption>,
) -> Result<impl IntoResponse, SupervisorError>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let format = opt.format();
    let now = Utc::now();
    let r = supervisor_metrics(now, ctx.name(), metrics::context::metrics()).await;
    Ok(toy_api_http_common::reply::into_response(
        &r,
        format,
        opt.indent(),
    ))
}

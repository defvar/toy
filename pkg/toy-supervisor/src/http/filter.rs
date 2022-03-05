use crate::error::SupervisorError;
use crate::http::Status;
use crate::supervisor::SupervisorContext;
use crate::{Request, RunTaskResponse};
use toy_api::common::ListOptionLike;
use toy_api::services::{ServiceSpec, ServiceSpecListOption};
use toy_api::task::{PendingTask, PostOption};
use toy_api_client::ApiClient;
use toy_api_http_common::bytes::Bytes;
use toy_api_http_common::warp;
use toy_api_http_common::warp::filters::BoxedFilter;
use toy_api_http_common::warp::hyper::StatusCode;
use toy_api_http_common::warp::reply::Reply;
use toy_api_http_common::warp::Filter;
use toy_core::error::ServiceError;
use toy_core::graph::Graph;

pub fn filters<C>(ctx: SupervisorContext<C>) -> BoxedFilter<(impl warp::Reply,)>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    index()
        .or(status(ctx.clone()))
        .or(services(ctx.clone()))
        .or(tasks(ctx.clone()))
        .or(shutdown(ctx))
        .boxed()
}

fn index() -> BoxedFilter<(impl warp::Reply,)> {
    warp::path::end().map(|| "Hello.").boxed()
}

fn status<C>(ctx: SupervisorContext<C>) -> BoxedFilter<(impl warp::Reply,)>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    warp::path!("status")
        .and(warp::get())
        .and(with_ctx(ctx))
        .and_then(handle_status)
        .boxed()
}

fn services<C>(ctx: SupervisorContext<C>) -> BoxedFilter<(impl warp::Reply,)>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    warp::path!("services")
        .and(warp::get())
        .and(toy_api_http_common::query::query_opt::<ServiceSpecListOption>())
        .and(with_ctx(ctx))
        .and_then(handle_services)
        .boxed()
}

fn tasks<C>(ctx: SupervisorContext<C>) -> BoxedFilter<(impl warp::Reply,)>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    warp::path!("tasks")
        .and(warp::post())
        .and(toy_api_http_common::query::query_opt::<PostOption>())
        .and(toy_api_http_common::body::bytes())
        .and(with_ctx(ctx))
        .and_then(handle_tasks)
        .boxed()
}

fn shutdown<C>(ctx: SupervisorContext<C>) -> BoxedFilter<(impl warp::Reply,)>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    warp::path!("shutdown")
        .and(warp::put())
        .and(with_ctx(ctx))
        .and_then(handle_shutdown)
        .boxed()
}

async fn handle_status<C>(ctx: SupervisorContext<C>) -> Result<impl warp::Reply, warp::Rejection>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let st = Status {
        name: ctx.name().to_owned(),
        started_at: ctx.started_at_str(),
        running_tasks: ctx.tasks().await,
    };
    match toy_pack_json::pack_to_string(&st) {
        Ok(v) => Ok(v.into_response()),
        Err(e) => {
            tracing::error!(err = %e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

async fn handle_services<C>(
    opt: Option<ServiceSpecListOption>,
    ctx: SupervisorContext<C>,
) -> Result<impl warp::Reply, warp::Rejection>
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
    let format = opt.as_ref().map(|x| x.common().format()).unwrap_or(None);
    let indent = opt.as_ref().map(|x| x.common().indent()).unwrap_or(None);

    Ok(toy_api_http_common::reply::into_response(
        &specs, format, indent,
    ))
}

async fn handle_tasks<C>(
    opt: Option<PostOption>,
    request: Bytes,
    mut ctx: SupervisorContext<C>,
) -> Result<impl warp::Reply, warp::Rejection>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    let format = opt.map(|x| x.format()).unwrap_or(None);
    let pending = toy_api_http_common::codec::decode::<_, PendingTask>(request, format)?;

    match pending.graph() {
        Some(graph) => {
            let v = toy_core::data::pack(graph)
                .map_err(|e| Into::<toy_api_http_common::Error>::into(e))?;
            let g = Graph::from(v).map_err(|e| Into::<SupervisorError>::into(e))?;
            tracing::debug!("{:?}", g);
            let (o_tx, _) = toy_core::oneshot::channel::<RunTaskResponse, ServiceError>();
            let req = Request::RunTask(pending.task_id(), g, o_tx);
            let _ = ctx.tx_mut().send_ok(req).await;
            Ok(StatusCode::CREATED)
        }
        None => Ok(StatusCode::NO_CONTENT),
    }
}

async fn handle_shutdown<C>(
    mut ctx: SupervisorContext<C>,
) -> Result<impl warp::Reply, warp::Rejection>
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

fn with_ctx<C>(
    ctx: SupervisorContext<C>,
) -> impl Filter<Extract = (SupervisorContext<C>,), Error = std::convert::Infallible> + Clone
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    warp::any().map(move || ctx.clone())
}

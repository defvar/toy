use crate::http::Status;
use crate::supervisor::SupervisorContext;
use crate::Request;
use toy_api_client::ApiClient;
use warp::filters::BoxedFilter;
use warp::hyper::StatusCode;
use warp::reply::Reply;
use warp::Filter;

pub fn filters<C>(ctx: SupervisorContext<C>) -> BoxedFilter<(impl warp::Reply,)>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    index().or(status(ctx.clone())).or(shutdown(ctx)).boxed()
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

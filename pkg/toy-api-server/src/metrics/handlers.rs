use crate::context::Context;
use crate::store::metrics::{CreateOption, MetricsStore, MetricsStoreOps};
use crate::ApiError;
use toy_api::common as api_common;
use toy_api::common::CommonPostResponse;
use toy_api::metrics::Metrics;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;
use toy_api_http_common::{codec, reply};
use toy_h::{HttpClient, StatusCode};

pub async fn post<T>(
    ctx: Context,
    opt: api_common::PostOption,
    request: Bytes,
    store: &impl MetricsStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let format = opt.format();
    let vec = codec::decode::<_, Vec<Metrics>>(request, format)?;

    for v in vec.into_iter().filter(|x| x.items().len() > 0) {
        match store
            .ops()
            .create(store.con().unwrap(), v, CreateOption::new())
            .await
        {
            Ok(()) => {}
            Err(e) => {
                tracing::error!("error:{:?}", e);
                return Err(ApiError::store_operation_failed(e));
            }
        }
    }
    let r = CommonPostResponse::with_code(StatusCode::CREATED.as_u16());
    let r = reply::into_response(&r, format, opt.indent());
    Ok((StatusCode::CREATED, r))
}

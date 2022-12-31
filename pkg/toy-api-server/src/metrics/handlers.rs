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
    let v = codec::decode::<_, Metrics>(request, format)?;

    match store
        .ops()
        .create(store.con().unwrap(), v, CreateOption::new())
        .await
    {
        Ok(()) => {
            let r = CommonPostResponse::with_code(StatusCode::CREATED.as_u16());
            let r = reply::into_response(&r, format, opt.indent());
            Ok((StatusCode::CREATED, r))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

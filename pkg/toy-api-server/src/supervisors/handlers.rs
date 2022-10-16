use crate::common::constants;
use crate::context::Context;
use crate::store::kv::{KvStore, Update, UpdateResult};
use crate::ApiError;
use chrono::Utc;
use toy_api::common::PostOption;
use toy_api::supervisors::Supervisor;
use toy_api_http_common::axum::response::IntoResponse;
use toy_h::{HttpClient, StatusCode};

pub async fn beat<T>(
    ctx: Context,
    store: &impl KvStore<T>,
    key: String,
    _opt: Option<PostOption>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let key = constants::generate_key(constants::SUPERVISORS_KEY_PREFIX, key);
    let now = Utc::now();
    let f = |v: Supervisor| Some(v.with_last_beat_time(now));
    match store
        .ops()
        .update(store.con().unwrap(), key.clone(), f)
        .await
    {
        Ok(UpdateResult::Update(_)) => Ok(StatusCode::OK),
        Ok(UpdateResult::NotFound) => Ok(StatusCode::NOT_FOUND),
        Ok(UpdateResult::None) => unreachable!(),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

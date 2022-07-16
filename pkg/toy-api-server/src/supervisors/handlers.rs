use crate::common::constants;
use crate::context::Context;
use crate::store::kv::{KvStore, Update, UpdateResult};
use chrono::Utc;
use toy_api::common::PostOption;
use toy_api::supervisors::Supervisor;
use toy_h::HttpClient;
use warp::http::StatusCode;

pub async fn beat<T>(
    key: String,
    ctx: Context,
    _opt: Option<PostOption>,
    store: impl KvStore<T>,
) -> Result<impl warp::Reply, warp::Rejection>
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
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

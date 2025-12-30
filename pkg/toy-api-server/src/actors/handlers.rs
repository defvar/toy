use crate::common::constants;
use crate::context::Context;
use crate::store::kv::{KvStore, Update, UpdateResult};
use crate::ApiError;
use chrono::Utc;
use toy_api::actors::{Actor, ActorBeatResponse};
use toy_api::common::PostOption;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::reply;
use toy_h::HttpClient;

pub async fn beat<T>(
    ctx: Context,
    store: &impl KvStore<T>,
    key: String,
    opt: PostOption,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);

    let key = constants::generate_key(constants::ACTORS_KEY_PREFIX, key);
    let now = Utc::now();
    let f = |v: Actor| Some(v.with_last_beat_time(now));
    match store
        .ops()
        .update(store.con().unwrap(), key.clone(), f)
        .await
    {
        Ok(UpdateResult::Update(_)) => {
            let r = ActorBeatResponse::Ok {
                last_beat_time: now,
            };
            Ok(reply::into_response(&r, opt.format(), opt.indent()))
        }
        Ok(UpdateResult::NotFound) => {
            let r = ActorBeatResponse::NotFound;
            Ok(reply::into_response(&r, opt.format(), opt.indent()))
        }
        Ok(UpdateResult::None) => unreachable!(),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

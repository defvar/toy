use crate::common;
use crate::context::Context;
use crate::store::kv::{Find, FindOption, KvStore, Put, PutOption, PutResult};
use chrono::Utc;
use toy_api::supervisors::{Supervisor, SupervisorStatus};
use toy_h::HttpClient;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn transition<T>(
    key: String,
    ctx: Context,
    store: impl KvStore<T>,
    status: SupervisorStatus,
) -> Result<impl warp::Reply, warp::Rejection>
where
    T: HttpClient,
{
    tracing::trace!("handle: {:?}", ctx);
    match store
        .ops()
        .find::<Supervisor>(
            store.con().unwrap(),
            common::constants::generate_key(common::constants::SUPERVISORS_KEY_PREFIX, &key),
            FindOption::new(),
        )
        .await
    {
        Ok(Some(resp)) => {
            let v = resp.into_value();
            if v.status() == status {
                return Ok(StatusCode::NOT_MODIFIED.into_response());
            }
            let v = v.with_status(status).with_last_transition_time(Utc::now());
            match store
                .ops()
                .put(
                    store.con().unwrap(),
                    common::constants::generate_key(
                        common::constants::SUPERVISORS_KEY_PREFIX,
                        &key,
                    ),
                    v,
                    PutOption::new().with_update_only(), //set version....
                )
                .await
            {
                Ok(PutResult::Update) => Ok(StatusCode::OK.into_response()),
                Ok(_) => unreachable!(),
                Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
            }
        }
        Ok(None) => Ok(StatusCode::NOT_FOUND.into_response()),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

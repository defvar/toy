use crate::common;
use crate::store::kv::{KvStore, List, ListOption, Put, PutOption, PutResult};
use crate::ApiError;
use chrono::Utc;
use toy_api::supervisors::{Supervisor, SupervisorStatus};
use toy_h::HttpClient;

pub async fn clean<T, Store>(store: Store, interval_mills: u64) -> Result<(), ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    loop {
        tracing::debug!("check pending supervisor");
        toy_rt::sleep(interval_mills).await;
        match store
            .ops()
            .list::<Supervisor>(
                store.con().unwrap(),
                common::constants::SUPERVISORS_KEY_PREFIX.to_string(),
                ListOption::new(),
            )
            .await
        {
            Ok(vec) => {
                let supervisors = vec
                    .into_iter()
                    .filter(|x| x.value().is_alive())
                    .collect::<Vec<_>>();
                for kvr in supervisors {
                    let version = kvr.version();
                    let sv = kvr.into_value();
                    let go = match sv.last_beat_time() {
                        Some(last_beat_time) => {
                            let since = Utc::now()
                                .signed_duration_since(last_beat_time.clone())
                                .num_seconds();
                            since > 30
                        }
                        None => true,
                    };
                    if go {
                        let name = sv.name().clone();
                        if let Err(e) =
                            put_sv(&store, sv.with_status(SupervisorStatus::NoContact), version)
                                .await
                        {
                            tracing::error!("clean supervisor failed, cause {:?}", e);
                        } else {
                            tracing::info!("cleaned supervisor, {}:NoContact", name);
                        }
                    }
                }
            }
            Err(e) => tracing::error!("clean supervisor failed, cause {:?}", e),
        }
    }
}

async fn put_sv<Store, T>(store: &Store, v: Supervisor, version: u64) -> Result<(), ApiError>
where
    Store: KvStore<T>,
    T: HttpClient,
{
    match store
        .ops()
        .put(
            store.con().unwrap(),
            common::constants::generate_key(common::constants::SUPERVISORS_KEY_PREFIX, v.name()),
            v,
            PutOption::new().with_update_only().with_version(version),
        )
        .await
    {
        Ok(PutResult::Update(_)) => Ok(()),
        Ok(_) => unreachable!(),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

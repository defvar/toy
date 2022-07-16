use crate::store::kv::KvStore;
use crate::toy_h::HttpClient;
use crate::{ApiError, ServerConfig};

mod role;
mod secret;

pub async fn initialize<T, Config>(
    config: &Config,
    store: impl KvStore<T> + 'static,
    client: T,
) -> Result<(), ApiError>
where
    T: HttpClient + 'static,
    Config: ServerConfig<T>,
{
    secret::initialize(config, &store).await?;
    role::initialize(config, &store).await?;

    crate::context::rbac::initialize(&store).await?;

    let s = store.clone();
    toy_rt::spawn_named(
        async move {
            tracing::info!("start watch context.");
            if let Err(e) = crate::context::rbac::sync_role_bindings(s).await {
                tracing::error!(err = ?e, "an error occured; when watch context.");
            }
        },
        "api-serve-sync_role",
    );

    let s = store.clone();
    toy_rt::spawn_named(
        async move {
            tracing::info!("start watch pending task.");
            if let Err(e) = crate::context::dispatcher::dispatch_task(s, client, 3000).await {
                tracing::error!(err = ?e, "an error occured; when watch pending task.");
            }
        },
        "api-serv-dispatch_task",
    );

    let s = store.clone();
    toy_rt::spawn_named(
        async move {
            tracing::info!("start watch pending supervisor.");
            if let Err(e) = crate::context::supervisor_cleaner::clean(s, 10000).await {
                tracing::error!(err = ?e, "an error occured; when watch pending supervisor.");
            }
        },
        "api-serv-supervisor_cleaner",
    );

    Ok(())
}

use crate::store::kv::KvStore;
use crate::toy_h::HttpClient;
use crate::{ApiError, ServerConfig};

mod role;
mod secret;

pub async fn initialize<T, Config>(config: &Config, store: impl KvStore<T>) -> Result<(), ApiError>
where
    T: HttpClient,
    Config: ServerConfig<T>,
{
    secret::initialize(config, store.clone()).await?;
    role::initialize(config, store.clone()).await?;
    Ok(())
}

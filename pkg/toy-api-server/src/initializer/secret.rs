use crate::common::constants;
use crate::store::kv::{KvStore, List, ListOption, Put, PutOption};
use crate::toy_h::HttpClient;
use crate::{ApiError, ServerConfig};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use toy_api::authentication::KeyPair;
use toy_api::authentication::Secret;

pub(crate) async fn initialize<T, Config>(
    config: &Config,
    store: &impl KvStore<T>,
) -> Result<(), ApiError>
where
    T: HttpClient,
    Config: ServerConfig,
{
    tracing::info!("initialize secret");

    let key = constants::generate_key(constants::SECRET_KEY_PREFIX, config.tls_secret_key());

    let private_key = from_file(&config.key_path())?;
    let pub_key = from_file(&config.pub_path())?;

    let v = KeyPair::new(config.tls_secret_key(), private_key, pub_key);

    store
        .ops()
        .put(
            store.con().unwrap(),
            key,
            Secret::KeyPair(v),
            PutOption::new(),
        )
        .await
        .map_err(|e| ApiError::server_initialize_failed(e))?;

    match store
        .ops()
        .list::<Secret>(
            store.con().unwrap(),
            constants::SECRET_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => {
            let map = v
                .into_iter()
                .map(|x| match x.into_value() {
                    Secret::KeyPair(key_pair) => (key_pair.kid().to_owned(), key_pair.clone()),
                })
                .collect::<HashMap<_, _>>();
            crate::context::server::set_key_pairs(map);
            Ok(())
        }
        Err(e) => Err(ApiError::server_initialize_failed(e)),
    }
}

fn from_file(path: &str) -> Result<String, ApiError> {
    let mut buf = Vec::new();
    let mut file = File::open(path).map_err(|e| ApiError::server_initialize_failed(e))?;
    match file.read_to_end(&mut buf) {
        Ok(_) => Ok(unsafe { String::from_utf8_unchecked(buf) }),
        Err(e) => Err(ApiError::server_initialize_failed(e)),
    }
}

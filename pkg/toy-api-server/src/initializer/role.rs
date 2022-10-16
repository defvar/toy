use crate::common::constants;
use crate::store::kv::{KvStore, Put, PutOption};
use crate::toy_h::HttpClient;
use crate::{ApiError, ServerConfig};
use toy_api::role::{Role, Rule};
use toy_api::role_binding::{RoleBinding, RoleBindingBuilder};

fn builtin_roles() -> Vec<Role> {
    vec![Role::new(
        "system",
        Some("system role"),
        vec![Rule::new(vec!["*".to_string()], vec!["*".to_string()])],
    )]
}

fn builtin_bindings() -> Vec<RoleBinding> {
    vec![RoleBindingBuilder::new("system")
        .role("system")
        .service_account("system")
        .build()]
}

pub(crate) async fn initialize<T, Config>(
    _config: &Config,
    store: &impl KvStore<T>,
) -> Result<(), ApiError>
where
    T: HttpClient,
    Config: ServerConfig,
{
    tracing::info!("initialize builtin role.");

    for r in builtin_roles() {
        let key = constants::generate_key(constants::ROLE_KEY_PREFIX, r.name());
        match store
            .ops()
            .put(store.con().unwrap(), key, r, PutOption::new())
            .await
        {
            Err(e) => return Err(ApiError::server_initialize_failed(e)),
            _ => (),
        }
    }

    tracing::info!("initialize builtin role binding.");

    for r in builtin_bindings() {
        let key = constants::generate_key(constants::ROLE_BINDING_KEY_PREFIX, r.name());
        match store
            .ops()
            .put(store.con().unwrap(), key, r, PutOption::new())
            .await
        {
            Err(e) => return Err(ApiError::server_initialize_failed(e)),
            _ => (),
        }
    }

    Ok(())
}

use crate::store::kv::{KvStore, KvWatchEventType, Watch, WatchOption};
use crate::store::kv::{List, ListOption};
use crate::toy_h::HttpClient;
use crate::ApiError;
use futures_util::stream::StreamExt;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use toy_api::role::Role;
use toy_api::role_binding::RoleBinding;

type RoleName = String;

static ROLE_BINDINGS: Lazy<AuthorizationCache> = Lazy::new(|| AuthorizationCache::new());

pub fn rules(user: &str) -> Option<Vec<Role>> {
    ROLE_BINDINGS.get(user)
}

pub struct AuthorizationCache {
    bindings: Mutex<HashMap<String, HashSet<RoleName>>>,
    roles: Mutex<HashMap<RoleName, Role>>,
}

impl AuthorizationCache {
    pub fn new() -> Self {
        Self {
            bindings: Mutex::new(HashMap::new()),
            roles: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, user: &str) -> Option<Vec<Role>> {
        let lock = self.bindings.lock().unwrap();
        if let Some(binding_roles) = lock.get(user) {
            let roles = self.roles.lock().unwrap();
            let r = binding_roles
                .iter()
                .filter_map(|x| roles.get(x))
                .cloned()
                .collect();
            Some(r)
        } else {
            None
        }
    }

    pub fn insert_roles(&self, roles: &[Role]) {
        let mut lock = self.roles.lock().unwrap();
        lock.extend(roles.iter().map(|x| (x.name().to_owned(), x.clone())));
    }

    pub fn insert_binding(&self, v: &RoleBinding) {
        let mut lock = self.bindings.lock().unwrap();
        v.subjects().iter().for_each(|sub| {
            lock.entry(sub.name().to_owned())
                .or_insert(HashSet::new())
                .insert(v.role().to_owned());
        });
    }

    pub fn delete_binding(&self, v: &RoleBinding) {
        let mut lock = self.bindings.lock().unwrap();
        v.subjects().iter().for_each(|sub| {
            if let Some(roles) = lock.get_mut(sub.name()) {
                roles.remove(v.role());
            }
        });
    }
}

pub async fn initialize<T>(store: &impl KvStore<T>) -> Result<(), ApiError>
where
    T: HttpClient,
{
    match store
        .ops()
        .list::<RoleBinding>(
            store.con().unwrap(),
            crate::common::constants::ROLE_BINDING_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => {
            v.iter().for_each(|x| ROLE_BINDINGS.insert_binding(x));
        }
        Err(e) => return Err(ApiError::server_initialize_failed(e)),
    };

    match store
        .ops()
        .list::<Role>(
            store.con().unwrap(),
            crate::common::constants::ROLE_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => {
            ROLE_BINDINGS.insert_roles(&v);
        }
        Err(e) => return Err(ApiError::server_initialize_failed(e)),
    };

    Ok(())
}

pub async fn sync_role_bindings<T, Store>(store: Store) -> Result<(), ApiError>
where
    Store: KvStore<T>,
{
    match store
        .ops()
        .watch::<RoleBinding>(
            store.con().unwrap(),
            crate::common::constants::ROLE_BINDING_KEY_PREFIX.to_string(),
            WatchOption::new(),
        )
        .await
    {
        Ok(st) => {
            let _ = st
                .for_each(|r| async move {
                    match r {
                        Ok(res) => res.into_values().into_iter().for_each(|v| match v.event() {
                            KvWatchEventType::PUT => {
                                let v = v.into_value();
                                ROLE_BINDINGS.insert_binding(&v);
                                tracing::debug!("update role binding. {:?}", v);
                            }
                            KvWatchEventType::DELETE => {
                                let v = v.into_value();
                                ROLE_BINDINGS.delete_binding(&v);
                                tracing::debug!("delete role binding. {:?}", v);
                            }
                        }),
                        Err(e) => {
                            tracing::error!("{:?}", e);
                        }
                    }
                })
                .await;
            Ok(())
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

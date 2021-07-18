use crate::common::constants;
use crate::store::kv::{KvStore, List, ListOption, Put, PutOption};
use crate::toy_h::HttpClient;
use crate::{ApiError, ServerConfig};
use std::collections::{HashMap, HashSet};
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
    store: impl KvStore<T>,
) -> Result<(), ApiError>
where
    T: HttpClient,
    Config: ServerConfig<T>,
{
    tracing::info!("initialize builtin role.");

    for r in builtin_roles() {
        let key = constants::generate_key(constants::ROLE_KEY_PREFIX, r.name().to_owned());
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
        let key = constants::generate_key(constants::ROLE_BINDING_KEY_PREFIX, r.name().to_owned());
        match store
            .ops()
            .put(store.con().unwrap(), key, r, PutOption::new())
            .await
        {
            Err(e) => return Err(ApiError::server_initialize_failed(e)),
            _ => (),
        }
    }

    let user_role_map = match store
        .ops()
        .list::<RoleBinding>(
            store.con().unwrap(),
            crate::common::constants::ROLE_BINDING_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => to_user_role_map(v),
        Err(e) => return Err(ApiError::server_initialize_failed(e)),
    };

    let rules = match store
        .ops()
        .list::<Role>(
            store.con().unwrap(),
            crate::common::constants::ROLE_KEY_PREFIX.to_string(),
            ListOption::new(),
        )
        .await
    {
        Ok(v) => to_user_rule_map(v, user_role_map),
        Err(e) => return Err(ApiError::server_initialize_failed(e)),
    };

    crate::context::server::set_rules(rules);

    Ok(())
}

fn to_user_role_map(v: Vec<RoleBinding>) -> HashMap<String, HashSet<String>> {
    let mut role_map = HashMap::<String, HashSet<String>>::new();
    v.iter().for_each(|binding| {
        binding.subjects().iter().for_each(|sub| {
            let e = role_map
                .entry(sub.name().to_owned())
                .or_insert(HashSet::new());
            let _ = e.insert(binding.role().to_owned());
        });
    });
    role_map
}

fn to_user_rule_map(
    roles: Vec<Role>,
    user_role_map: HashMap<String, HashSet<String>>,
) -> HashMap<String, Vec<Rule>> {
    let map = roles
        .iter()
        .map(|x| (x.name(), x.rules()))
        .collect::<HashMap<_, _>>();
    let mut result = HashMap::<String, Vec<Rule>>::new();
    user_role_map.iter().for_each(|(user, roles)| {
        roles.iter().for_each(|r| {
            map.get(r.as_str()).map(|rules| {
                let _ = result
                    .entry(user.to_string())
                    .and_modify(|x| x.extend_from_slice(rules))
                    .or_insert(rules.to_vec());
            });
        });
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_user_rule_map() {
        let b = get_test_role_bindings();
        let role_map = to_user_role_map(b);
        let roles = get_test_roles();
        let r = to_user_rule_map(roles, role_map);

        let vec = r.get("user-1").unwrap();
        let (resource, verbs) = vec.iter().fold(
            (HashSet::new(), HashSet::new()),
            |(mut resouces, mut verbs), v| {
                resouces.extend(v.resources().clone());
                verbs.extend(v.verbs().clone());
                (resouces, verbs)
            },
        );
        let mut resource_vec = resource.into_iter().collect::<Vec<_>>();
        resource_vec.sort();
        let mut verbs_vec = verbs.into_iter().collect::<Vec<_>>();
        verbs_vec.sort();

        assert_eq!(resource_vec, vec!["aiueo".to_string()]);
        assert_eq!(verbs_vec, vec!["GET".to_string(), "POST".to_string()]);
    }

    #[test]
    fn test_to_user_role_map() {
        let b = get_test_role_bindings();
        let r = to_user_role_map(b);

        let vec: Vec<_> = r.get("system-user").unwrap().into_iter().collect();
        assert_eq!(vec, vec!["system-role"]);

        let mut vec: Vec<_> = r.get("user-1").unwrap().into_iter().collect::<Vec<_>>();
        vec.sort();
        assert_eq!(vec, vec!["user-role", "user-role-2"]);

        let vec: Vec<_> = r.get("user-2").unwrap().into_iter().collect();
        assert_eq!(vec, vec!["user-role"]);
    }

    fn get_test_role_bindings() -> Vec<RoleBinding> {
        let b = vec![
            RoleBindingBuilder::new("system-bind")
                .role("system-role")
                .service_account("system-user")
                .build(),
            RoleBindingBuilder::new("user-bind")
                .role("user-role")
                .user("user-1")
                .user("user-2")
                .build(),
            RoleBindingBuilder::new("user-bind-2")
                .role("user-role-2")
                .user("user-1")
                .build(),
        ];
        b
    }

    fn get_test_roles() -> Vec<Role> {
        vec![
            Role::new(
                "user-role",
                None,
                vec![Rule::new(
                    vec!["aiueo".to_string()],
                    vec!["GET".to_string()],
                )],
            ),
            Role::new(
                "user-role-2",
                None,
                vec![Rule::new(
                    vec!["aiueo".to_string()],
                    vec!["POST".to_string()],
                )],
            ),
            Role::new(
                "system-role",
                None,
                vec![Rule::new(vec!["*".to_string()], vec!["*".to_string()])],
            ),
        ]
    }
}

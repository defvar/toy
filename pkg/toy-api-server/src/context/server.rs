use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use toy_api::authentication::KeyPair;
use toy_api::role::Rule;

static KEY_PAIR_CACHES: Lazy<Mutex<HashMap<String, KeyPair>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static RULE_CACHES: Lazy<Mutex<HashMap<String, Vec<Rule>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn key_pair(kid: &str) -> Option<KeyPair> {
    let lock = KEY_PAIR_CACHES.lock().unwrap();
    lock.get(kid).map(|x| x.clone())
}

pub fn set_key_pairs(pairs: HashMap<String, KeyPair>) {
    let count = pairs.keys().count();
    let mut lock = KEY_PAIR_CACHES.lock().unwrap();
    pairs.into_iter().for_each(move |(k, v)| {
        let _ = lock.insert(k, v);
    });
    tracing::info!("set key pairs cache. count:{}", count);
}

pub fn rules(user: &str) -> Option<Vec<Rule>> {
    let lock = RULE_CACHES.lock().unwrap();
    lock.get(user).map(|x| x.clone())
}

pub fn set_rules(rules: HashMap<String, Vec<Rule>>) {
    let count = rules.keys().count();
    let mut lock = RULE_CACHES.lock().unwrap();
    rules.into_iter().for_each(move |(k, v)| {
        let _ = lock.insert(k, v);
    });
    tracing::info!("set rules cache. count:{}", count);
}

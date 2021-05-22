use std::collections::HashMap;
use std::sync::Mutex;

pub struct InmemoryCache<K, V, F> {
    raw: Mutex<HashMap<K, V>>,
    sync_func: F,
}

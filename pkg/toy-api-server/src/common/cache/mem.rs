use crate::common::cache::TypedCache;
use async_trait::async_trait;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Mutex;

pub struct InmemoryCache<K, V, F, Fut> {
    raw: Mutex<HashMap<K, V>>,
    sync_func: F,
    _t: PhantomData<Fut>,
}

impl<K, V, F, Fut> InmemoryCache<K, V, F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = HashMap<K, V>> + Send,
{
    pub fn new(init: HashMap<K, V>, sync_func: F) -> Self {
        Self {
            raw: Mutex::new(init),
            sync_func,
            _t: PhantomData,
        }
    }
}

#[async_trait]
impl<K, V, F, Fut> TypedCache<K, V> for InmemoryCache<K, V, F, Fut>
where
    K: Send + Clone + Eq + Hash,
    V: Send + Clone,
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = HashMap<K, V>> + Send,
{
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        let lock = self.raw.lock().unwrap();
        lock.get(key).map(|x| x.clone())
    }

    async fn sync(&mut self) -> Result<(), ()> {
        //TODO: remaind last sync time....?

        let map = &(self.sync_func)().await;
        {
            let mut lock = self.raw.lock().unwrap();
            map.into_iter().for_each(move |(k, v)| {
                let _ = lock.insert(k.clone(), v.clone());
            });
        }
        Ok(())
    }
}

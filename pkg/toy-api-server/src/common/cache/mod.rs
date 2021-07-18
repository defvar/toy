use async_trait::async_trait;
use std::borrow::Borrow;
use std::hash::Hash;

mod mem;
pub use mem::InmemoryCache;

#[async_trait]
pub trait TypedCache<K, V>
where
    V: Send + Clone,
{
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + Eq + Hash;

    async fn sync(&mut self) -> Result<(), ()>;
}

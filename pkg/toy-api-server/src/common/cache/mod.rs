use std::borrow::Borrow;
use std::hash::Hash;

mod mem;

pub use mem::InmemoryCache;

pub trait TypedCache<K, V> {
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + Eq + Hash;
}

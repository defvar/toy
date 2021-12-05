use crate::Map;
use serde::{Serialize, Serializer};
use std::hash::Hash;

impl<K, V> Serialize for Map<K, V>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

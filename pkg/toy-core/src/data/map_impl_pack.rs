use crate::data::Map;
use std::hash::Hash;
use toy_pack::ser::{Serializable, Serializer};

impl<K, V> Serializable for Map<K, V>
where
    K: Serializable + Eq + Hash,
    V: Serializable,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

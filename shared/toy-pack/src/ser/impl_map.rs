use std::collections::HashMap;
use std::hash::Hash;

use super::{Serializable, Serializer};

impl<K, V> Serializable for HashMap<K, V>
    where K: Serializable + Eq + Hash,
          V: Serializable,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        serializer.collect_map(self)
    }
}

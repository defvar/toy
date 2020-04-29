use crate::data::Map;
use core::marker::PhantomData;
use std::hash::Hash;
use toy_pack::deser::{Deserializable, DeserializeMapOps, Deserializer, Visitor};

impl<'toy, K, V> Deserializable<'toy> for Map<K, V>
where
    K: Deserializable<'toy>,
    V: Deserializable<'toy>,
    K: Eq + Hash,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct __Visitor<K, V> {
            t: PhantomData<Map<K, V>>,
        }

        impl<'toy, K, V> Visitor<'toy> for __Visitor<K, V>
        where
            K: Deserializable<'toy>,
            V: Deserializable<'toy>,
            K: Eq + Hash,
        {
            type Value = Map<K, V>;

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: DeserializeMapOps<'toy>,
            {
                let mut values = Map::with_capacity(map.size_hint().unwrap_or(256));
                while let Some(key) = map.next_key::<K>()? {
                    let v = map.next_value::<V>()?;
                    values.insert(key, v);
                }
                Ok(values)
            }
        }

        let visitor: __Visitor<K, V> = __Visitor { t: PhantomData };
        deserializer.deserialize_map(visitor)
    }
}

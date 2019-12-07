use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use super::{Deserializable, DeserializeMapOps, Deserializer, Visitor};

// Implementation `Deserializable` for `HashMap<K, V>`
//
impl<'toy, K, V> Deserializable<'toy> for HashMap<K, V>
    where K: Deserializable<'toy>,
          V: Deserializable<'toy>,
          K::Value: Eq + Hash
{
    type Value = HashMap<K::Value, V::Value>;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'toy> {
        struct __Visitor<K, V> {
            t: PhantomData<HashMap<K, V>>,
        }

        impl<'toy, K, V> Visitor<'toy> for __Visitor<K, V>
            where K: Deserializable<'toy>,
                  V: Deserializable<'toy>,
                  K::Value: Eq + Hash
        {
            type Value = HashMap<K::Value, V::Value>;

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where A: DeserializeMapOps<'toy>
            {
                let size = map.size_hint().unwrap_or(256);
                let mut values: HashMap<K::Value, V::Value> = HashMap::with_capacity(size);
                while let Some(key) = map.next_key::<K>()? {
                    let v = map.next_value::<V>()?;
                    values.insert(key, v);
                }
                Ok(values)
            }
        }

        let visitor: __Visitor<K, V> = __Visitor {
            t: PhantomData,
        };
        deserializer.deserialize_map(visitor)
    }
}

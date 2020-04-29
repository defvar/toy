use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;

use super::{Deserializable, DeserializeMapOps, Deserializer, Visitor};

macro_rules! map_impl {
    (
        $ty: ident < K, V >,
        $kbound1: ident $(+ $kbound2:ident)*,
        $access: ident,
        $with_capacity: expr
    ) => {
        impl<'toy, K, V> Deserializable<'toy> for $ty<K, V>
        where
            K: Deserializable<'toy>,
            V: Deserializable<'toy>,
            K: $kbound1 $(+ $kbound2)*,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'toy>,
            {
                struct __Visitor<K, V> {
                    t: PhantomData<$ty<K, V>>,
                }

                impl<'toy, K, V> Visitor<'toy> for __Visitor<K, V>
                where
                    K: Deserializable<'toy>,
                    V: Deserializable<'toy>,
                    K: $kbound1 $(+ $kbound2)*,
                {
                    type Value = $ty<K, V>;

                    fn visit_map<A>(self, mut $access: A) -> Result<Self::Value, A::Error>
                    where
                        A: DeserializeMapOps<'toy>,
                    {
                        let mut values = $with_capacity;
                        while let Some(key) = $access.next_key::<K>()? {
                            let v = $access.next_value::<V>()?;
                            values.insert(key, v);
                        }
                        Ok(values)
                    }
                }

                let visitor: __Visitor<K, V> = __Visitor { t: PhantomData };
                deserializer.deserialize_map(visitor)
            }
        }
    };
}

map_impl!(
 BTreeMap<K, V>,
 Ord,
 map,
 BTreeMap::new()
);

map_impl!(
 HashMap<K, V>,
 Eq + Hash,
 map,
 HashMap::with_capacity(map.size_hint().unwrap_or(256))
);

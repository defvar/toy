use std::marker::PhantomData;

use super::{Deserializable, DeserializeSeqOps, Deserializer, Visitor};

// Implementation `Deserializable` for `Vec<T>`
//
impl<'toy, T> Deserializable<'toy> for Vec<T>
where
    T: Deserializable<'toy>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct __Visitor<T> {
            t: PhantomData<T>,
        }

        impl<'toy, T> Visitor<'toy> for __Visitor<T>
        where
            T: Deserializable<'toy>,
        {
            type Value = Vec<T>;

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: DeserializeSeqOps<'toy>,
            {
                let size = seq.size_hint().unwrap_or(256);
                let mut vec: Vec<T> = Vec::with_capacity(size);
                while let Some(item) = seq.next::<T>()? {
                    vec.push(item);
                }
                Ok(vec)
            }
        }

        let visitor: __Visitor<T> = __Visitor { t: PhantomData };
        deserializer.deserialize_seq(visitor)
    }
}

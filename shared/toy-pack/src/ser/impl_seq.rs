use super::{Serializable, Serializer};

impl<T> Serializable for Vec<T>
where
    T: Serializable,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

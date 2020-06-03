use super::{Schema, SchemaVisitor, WrapTypes};
use std::collections::HashMap;
use std::path::PathBuf;

impl<T> Schema for Option<T>
where
    T: Schema,
{
    fn scan<V>(name: &str, mut visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        visitor.visit_wrap_type::<T>(name, WrapTypes::Option)
    }
}

impl<T> Schema for Box<T>
where
    T: Schema,
{
    fn scan<V>(name: &str, visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        T::scan(name, visitor)
    }
}

impl<K, Val> Schema for HashMap<K, Val>
where
    K: Schema,
    Val: Schema,
{
    fn scan<V>(name: &str, mut visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        visitor.visit_map_type::<K, Val>(name)
    }
}

impl<T> Schema for Vec<T>
where
    T: Schema,
{
    fn scan<V>(name: &str, mut visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        visitor.visit_wrap_type::<T>(name, WrapTypes::Vec)
    }
}

impl Schema for PathBuf {
    fn scan<V>(name: &str, visitor: V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        String::scan(name, visitor)
    }
}

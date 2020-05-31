use super::{Schema, SchemaVisitor, WrapTypes};
use std::collections::HashMap;

impl<T> Schema for Option<T>
where
    T: Schema,
{
    fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
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
    fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
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
    fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
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
    fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
    where
        V: SchemaVisitor,
    {
        visitor.visit_wrap_type::<T>(name, WrapTypes::Vec)
    }
}

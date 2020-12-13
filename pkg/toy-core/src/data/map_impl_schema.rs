use crate::data::Map;
use toy_pack::schema::{Schema, SchemaVisitor};

impl<K, Val> Schema for Map<K, Val>
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

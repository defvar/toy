use super::error::Error;
use super::{PrimitiveTypes, WrapTypes};
use crate::schema::Schema;

pub trait SchemaVisitor: Sized {
    type Value;
    type Error: Error;
    type StructVisitor: StructVisitor<Value = Self::Value, Error = Self::Error>;
    type TupleVariantVisitor: TupleVariantVisitor<Value = Self::Value, Error = Self::Error>;
    type EnumVisitor: EnumVisitor<
        Value = Self::Value,
        Error = Self::Error,
        TupleVariantVisitor = Self::TupleVariantVisitor,
    >;

    fn visit(&mut self, name: &'static str, tp: PrimitiveTypes)
        -> Result<Self::Value, Self::Error>;

    fn visit_wrap_type<T>(
        &mut self,
        name: &'static str,
        wrap: WrapTypes,
    ) -> Result<Self::Value, Self::Error>
    where
        T: Schema;

    fn visit_map_type<K, V>(&mut self, name: &'static str) -> Result<Self::Value, Self::Error>
    where
        K: Schema,
        V: Schema;

    fn struct_visitor(&mut self, name: &'static str) -> Result<Self::StructVisitor, Self::Error>;

    fn enum_visitor(
        &mut self,
        name: &'static str,
        enum_name: &'static str,
    ) -> Result<Self::EnumVisitor, Self::Error>;
}

pub trait StructVisitor {
    type Value;
    type Error: Error;

    fn field<T>(&mut self, name: &'static str) -> Result<(), Self::Error>
    where
        T: Schema;

    fn end(self) -> Result<Self::Value, Self::Error>;
}

pub trait TupleVariantVisitor {
    type Value;
    type Error: Error;

    fn tuple_variant_arg<T>(
        &mut self,
        enum_name: &'static str,
        variant: &'static str,
        arg_idx: u32,
    ) -> Result<(), Self::Error>
    where
        T: Schema;

    fn end(self) -> Result<Self::Value, Self::Error>;
}

pub trait EnumVisitor {
    type Value;
    type Error: Error;
    type TupleVariantVisitor: TupleVariantVisitor<Value = Self::Value, Error = Self::Error>;

    fn unit_variant(
        &mut self,
        name: &'static str,
        variant: &'static str,
    ) -> Result<(), Self::Error>;

    fn tuple_variant_visitor(
        &mut self,
        name: &'static str,
        variant: &'static str,
    ) -> Result<Self::TupleVariantVisitor, Self::Error>;

    fn variant(
        &mut self,
        name: &'static str,
        variant: &'static str,
        v: Self::Value,
    ) -> Result<(), Self::Error>;

    fn end(self) -> Result<Self::Value, Self::Error>;
}

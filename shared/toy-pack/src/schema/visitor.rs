use super::error::Error;
use super::{PrimitiveTypes, WrapTypes};
use crate::schema::Schema;

/// The trait to scan target schema.
///
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

    /// Get visitor for struct.
    fn struct_visitor(&mut self, name: &'static str) -> Result<Self::StructVisitor, Self::Error>;

    /// Get visitor for enum.
    fn enum_visitor(
        &mut self,
        name: &'static str,
        enum_name: &'static str,
    ) -> Result<Self::EnumVisitor, Self::Error>;
}

/// The trait to scan target schema.
///
pub trait StructVisitor {
    type Value;
    type Error: Error;

    /// Add field.
    fn field<T>(&mut self, name: &'static str) -> Result<(), Self::Error>
    where
        T: Schema;

    /// End visit and create Schema.
    fn end(self) -> Result<Self::Value, Self::Error>;
}

/// The trait to scan target schema
/// for scanning tuple varint enum fields.
///
/// ```
/// enum ABC{
///   A(u32, u32)
/// }
/// ```
///
pub trait TupleVariantVisitor {
    type Value;
    type Error: Error;

    /// Add arg.
    /// ```
    /// enum T {
    ///   A(/* call first */ u32, /* call second */ u8),
    /// }
    /// ```
    fn tuple_variant_arg<T>(
        &mut self,
        enum_name: &'static str,
        variant: &'static str,
        arg_idx: u32,
    ) -> Result<(), Self::Error>
    where
        T: Schema;

    /// End visit and create Schema.
    fn end(self) -> Result<Self::Value, Self::Error>;
}

/// The trait to scan target schema
/// for scanning enum fields.
///
pub trait EnumVisitor {
    type Value;
    type Error: Error;
    type TupleVariantVisitor: TupleVariantVisitor<Value = Self::Value, Error = Self::Error>;

    /// Add unit variant.
    ///
    /// ```
    /// enum E {
    ///     A,
    ///     B,
    /// }
    /// ```
    fn unit_variant(
        &mut self,
        name: &'static str,
        variant: &'static str,
    ) -> Result<(), Self::Error>;

    /// Get new visitor for tuple variant.
    ///
    fn tuple_variant_visitor(
        &mut self,
        name: &'static str,
        variant: &'static str,
    ) -> Result<Self::TupleVariantVisitor, Self::Error>;

    /// Add any variant.
    fn variant(
        &mut self,
        name: &'static str,
        variant: &'static str,
        v: Self::Value,
    ) -> Result<(), Self::Error>;

    /// End visit and create Schema.
    fn end(self) -> Result<Self::Value, Self::Error>;
}

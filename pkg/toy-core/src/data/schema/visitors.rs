//! Schema Visitor for JsonSchema.

use crate::data::schema::builders::{ArrayBuilder, ObjectBuilder, OneOfBuilder, SchemaBuilders};
use crate::data::schema::{JsonSchema, SchemaScanError, SchemaTypes};
use toy_pack::schema::{
    EnumVisitor, PrimitiveTypes, Schema, SchemaVisitor, StructVisitor, TupleVariantVisitor,
    WrapTypes,
};

pub struct JsonSchemaVisitor;

pub struct JsonSchemaStructVisitor {
    builder: ObjectBuilder,
}

pub struct JsonSchemaEnumVisitor {
    builder: OneOfBuilder,
}

pub struct JsonSchemaTupleVariantVisitor {
    builder: ArrayBuilder,
    variant: String,
}

impl SchemaVisitor for JsonSchemaVisitor {
    type Value = JsonSchema;
    type Error = SchemaScanError;
    type StructVisitor = JsonSchemaStructVisitor;
    type TupleVariantVisitor = JsonSchemaTupleVariantVisitor;
    type EnumVisitor = JsonSchemaEnumVisitor;

    fn visit(&mut self, _name: &str, tp: PrimitiveTypes) -> Result<Self::Value, Self::Error> {
        Ok(match tp {
            PrimitiveTypes::Boolean => JsonSchema::from_types(SchemaTypes::Boolean),
            PrimitiveTypes::U8 => SchemaBuilders::integer_builder().u8().build(),
            PrimitiveTypes::U16 => SchemaBuilders::integer_builder().u16().build(),
            PrimitiveTypes::U32 => SchemaBuilders::integer_builder().u32().build(),
            PrimitiveTypes::U64 => SchemaBuilders::integer_builder().u64().build(),
            PrimitiveTypes::USize => SchemaBuilders::integer_builder().usize().build(),
            PrimitiveTypes::I8 => SchemaBuilders::integer_builder().i8().build(),
            PrimitiveTypes::I16 => SchemaBuilders::integer_builder().i16().build(),
            PrimitiveTypes::I32 => SchemaBuilders::integer_builder().i32().build(),
            PrimitiveTypes::I64 => SchemaBuilders::integer_builder().i64().build(),
            PrimitiveTypes::ISize => SchemaBuilders::integer_builder().isize().build(),
            PrimitiveTypes::F32 => SchemaBuilders::number_builder().f32().build(),
            PrimitiveTypes::F64 => SchemaBuilders::number_builder().f64().build(),
            PrimitiveTypes::String | PrimitiveTypes::Str | PrimitiveTypes::Char => {
                JsonSchema::from_types(SchemaTypes::String)
            }
        })
    }

    fn visit_wrap_type<T>(
        &mut self,
        name: &str,
        wrap: WrapTypes,
    ) -> Result<Self::Value, Self::Error>
    where
        T: Schema,
    {
        let s = T::scan(name, JsonSchemaVisitor)?;
        let r = match wrap {
            WrapTypes::Option => s.into_optional(),
            WrapTypes::Vec => SchemaBuilders::array_builder().push(s).build(),
        };
        Ok(r)
    }

    fn visit_map_type<K, V>(&mut self, name: &str) -> Result<Self::Value, Self::Error>
    where
        K: Schema,
        V: Schema,
    {
        let k = K::scan(name, JsonSchemaVisitor)?;
        if let Some(tp) = k.tp() {
            assert_eq!(tp, SchemaTypes::String, "map key must be a String type.");
        }
        let v = V::scan(name, JsonSchemaVisitor)?;
        Ok(SchemaBuilders::map_builder().additional(v).build())
    }

    fn struct_visitor(&mut self, _name: &str) -> Result<Self::StructVisitor, Self::Error> {
        let r = JsonSchemaStructVisitor {
            builder: SchemaBuilders::object_builder(),
        };
        Ok(r)
    }

    fn enum_visitor(
        &mut self,
        _name: &str,
        _enum_name: &'static str,
    ) -> Result<Self::EnumVisitor, Self::Error> {
        let r = JsonSchemaEnumVisitor {
            builder: SchemaBuilders::one_of_builder(),
        };
        Ok(r)
    }
}

impl EnumVisitor for JsonSchemaEnumVisitor {
    type Value = JsonSchema;
    type Error = SchemaScanError;
    type TupleVariantVisitor = JsonSchemaTupleVariantVisitor;

    fn unit_variant(&mut self, _name: &str, variant: &'static str) -> Result<(), Self::Error> {
        let s = SchemaBuilders::const_builder().value(variant).build();
        self.builder.push(s);
        Ok(())
    }

    fn tuple_variant_visitor(
        &mut self,
        _name: &str,
        variant: &'static str,
    ) -> Result<Self::TupleVariantVisitor, Self::Error> {
        Ok(JsonSchemaTupleVariantVisitor {
            builder: SchemaBuilders::array_builder(),
            variant: variant.to_string(),
        })
    }

    fn variant(
        &mut self,
        _name: &str,
        _variant: &'static str,
        v: Self::Value,
    ) -> Result<(), Self::Error> {
        self.builder.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Value, Self::Error> {
        Ok(self.builder.build())
    }
}

impl TupleVariantVisitor for JsonSchemaTupleVariantVisitor {
    type Value = JsonSchema;
    type Error = SchemaScanError;

    fn tuple_variant_arg<T>(
        &mut self,
        _enum_name: &'static str,
        variant: &'static str,
        _arg_idx: u32,
    ) -> Result<(), Self::Error>
    where
        T: Schema,
    {
        let s = T::scan(variant, JsonSchemaVisitor)?;
        self.builder.push(s);
        Ok(())
    }

    fn end(self) -> Result<Self::Value, Self::Error> {
        let r = SchemaBuilders::object_builder()
            .property(self.variant.as_str(), self.builder.build())
            .build();
        Ok(r)
    }
}

impl StructVisitor for JsonSchemaStructVisitor {
    type Value = JsonSchema;
    type Error = SchemaScanError;

    fn field<T>(&mut self, name: &str) -> Result<(), Self::Error>
    where
        T: Schema,
    {
        let p = T::scan(name, JsonSchemaVisitor)?;
        self.builder.property(name, p);
        Ok(())
    }

    fn end(self) -> Result<Self::Value, Self::Error> {
        Ok(self.builder.build())
    }
}

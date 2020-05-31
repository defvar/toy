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

    fn visit(
        &mut self,
        _name: &'static str,
        tp: PrimitiveTypes,
    ) -> Result<Self::Value, Self::Error> {
        match tp {
            PrimitiveTypes::Boolean => Ok(JsonSchema::from_types(SchemaTypes::Boolean)),
            PrimitiveTypes::U8
            | PrimitiveTypes::U16
            | PrimitiveTypes::U32
            | PrimitiveTypes::U64
            | PrimitiveTypes::USize
            | PrimitiveTypes::I8
            | PrimitiveTypes::I16
            | PrimitiveTypes::I32
            | PrimitiveTypes::I64
            | PrimitiveTypes::ISize => Ok(JsonSchema::from_types(SchemaTypes::Integer)),
            PrimitiveTypes::F32 | PrimitiveTypes::F64 => {
                Ok(JsonSchema::from_types(SchemaTypes::Number))
            }
            PrimitiveTypes::String | PrimitiveTypes::Str => {
                Ok(JsonSchema::from_types(SchemaTypes::String))
            }
        }
    }

    fn visit_wrap_type<T>(
        &mut self,
        name: &'static str,
        wrap: WrapTypes,
    ) -> Result<Self::Value, Self::Error>
    where
        T: Schema,
    {
        let s = T::scan(name, self)?;
        let r = match wrap {
            WrapTypes::Option => s.into_optional(),
            WrapTypes::Vec => SchemaBuilders::array_builder().push(s).build(),
        };
        Ok(r)
    }

    fn visit_map_type<K, V>(&mut self, name: &'static str) -> Result<Self::Value, Self::Error>
    where
        K: Schema,
        V: Schema,
    {
        let k = K::scan(name, self)?;
        if let Some(tp) = k.tp() {
            assert_eq!(tp, SchemaTypes::String, "map key must be a String type.");
        }
        let v = V::scan(name, self)?;
        Ok(SchemaBuilders::map_builder().additional(v).build())
    }

    fn struct_visitor(&mut self, _name: &'static str) -> Result<Self::StructVisitor, Self::Error> {
        let r = JsonSchemaStructVisitor {
            builder: SchemaBuilders::object_builder(),
        };
        Ok(r)
    }

    fn enum_visitor(
        &mut self,
        _name: &'static str,
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

    fn unit_variant(
        &mut self,
        _name: &'static str,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        let s = SchemaBuilders::const_builder().value(variant).build();
        self.builder.push(s);
        Ok(())
    }

    fn tuple_variant_visitor(
        &mut self,
        _name: &'static str,
        variant: &'static str,
    ) -> Result<Self::TupleVariantVisitor, Self::Error> {
        Ok(JsonSchemaTupleVariantVisitor {
            builder: SchemaBuilders::array_builder(),
            variant: variant.to_string(),
        })
    }

    fn variant(
        &mut self,
        _name: &'static str,
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
        let mut v = JsonSchemaVisitor;
        let s = T::scan(variant, &mut v)?;
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

    fn field<T>(&mut self, name: &'static str) -> Result<(), Self::Error>
    where
        T: Schema,
    {
        let mut v = JsonSchemaVisitor;
        let p = T::scan(name, &mut v)?;
        self.builder.property(name, p);
        Ok(())
    }

    fn end(self) -> Result<Self::Value, Self::Error> {
        Ok(self.builder.build())
    }
}

use crate::data::schema::json_schema::RangeValue;
use crate::data::schema::{JsonSchema, SchemaTypes};
use crate::data::Map;

pub struct SchemaBuilders;

#[derive(Clone)]
pub struct ConstBuilder {
    schema: JsonSchema,
}

#[derive(Clone)]
pub struct ObjectBuilder {
    schema: JsonSchema,
}

#[derive(Clone)]
pub struct MapBuilder {
    schema: JsonSchema,
}

#[derive(Clone)]
pub struct ArrayBuilder {
    schema: JsonSchema,
}

#[derive(Clone)]
pub struct OneOfBuilder {
    schema: JsonSchema,
}

#[derive(Clone)]
pub struct IntegerBuilder {
    schema: JsonSchema,
}

#[derive(Clone)]
pub struct NumberBuilder {
    schema: JsonSchema,
}

impl SchemaBuilders {
    pub fn const_builder() -> ConstBuilder {
        ConstBuilder::new()
    }

    pub fn object_builder() -> ObjectBuilder {
        ObjectBuilder::new()
    }

    pub fn map_builder() -> MapBuilder {
        MapBuilder::new()
    }

    pub fn array_builder() -> ArrayBuilder {
        ArrayBuilder::new()
    }

    pub fn one_of_builder() -> OneOfBuilder {
        OneOfBuilder::new()
    }

    pub fn integer_builder() -> IntegerBuilder {
        IntegerBuilder::new()
    }

    pub fn number_builder() -> NumberBuilder {
        NumberBuilder::new()
    }
}

impl ConstBuilder {
    pub fn new() -> ConstBuilder {
        ConstBuilder {
            schema: JsonSchema::new(),
        }
    }

    pub fn build(&self) -> JsonSchema {
        self.schema.clone()
    }

    pub fn value(&mut self, v: &str) -> &mut Self {
        self.schema.const_ = Some(v.to_string());
        self
    }
}

impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder {
            schema: JsonSchema::from_types(SchemaTypes::Object),
        }
    }

    pub fn build(&self) -> JsonSchema {
        let required = match &self.schema.properties {
            Some(ref map) => map
                .iter()
                .filter(|(_, v)| !v.is_optional)
                .map(|(k, _)| k.clone())
                .collect(),
            None => vec![],
        };
        self.schema.clone().into_required(required)
    }

    pub fn property(&mut self, name: &str, prop: JsonSchema) -> &mut Self {
        match self.schema.properties {
            Some(ref mut v) => {
                v.insert(name.to_string(), prop);
            }
            None => {
                let map = {
                    let mut m = Map::new();
                    m.insert(name.to_string(), prop);
                    m
                };
                self.schema.properties = Some(map);
            }
        };
        self
    }
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder {
            schema: JsonSchema::from_types(SchemaTypes::Object),
        }
    }

    pub fn build(&self) -> JsonSchema {
        self.schema.clone()
    }

    pub fn additional(&mut self, prop: JsonSchema) -> &mut Self {
        self.schema.additional_properties = Some(Box::new(prop));
        self
    }
}

impl ArrayBuilder {
    pub fn new() -> ArrayBuilder {
        ArrayBuilder {
            schema: JsonSchema::from_types(SchemaTypes::Array),
        }
    }

    pub fn build(&self) -> JsonSchema {
        self.schema.clone()
    }

    pub fn push(&mut self, prop: JsonSchema) -> &mut Self {
        match self.schema.items {
            Some(ref mut v) => {
                v.push(prop);
            }
            None => {
                self.schema.items = Some(vec![prop]);
            }
        }
        self
    }
}

impl OneOfBuilder {
    pub fn new() -> OneOfBuilder {
        OneOfBuilder {
            schema: JsonSchema::new(),
        }
    }

    pub fn build(&self) -> JsonSchema {
        self.schema.clone()
    }

    pub fn push(&mut self, prop: JsonSchema) -> &mut Self {
        match self.schema.one_of {
            Some(ref mut v) => {
                v.push(prop);
            }
            None => {
                self.schema.one_of = Some(vec![prop]);
            }
        };
        self
    }
}

impl IntegerBuilder {
    pub fn new() -> IntegerBuilder {
        IntegerBuilder {
            schema: JsonSchema::from_types(SchemaTypes::Integer),
        }
    }

    pub fn build(&self) -> JsonSchema {
        self.schema.clone()
    }

    pub fn i8(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::I64(i8::min_value() as i64));
        self.schema.maximum = Some(RangeValue::I64(i8::max_value() as i64));
        self
    }

    pub fn i16(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::I64(i16::min_value() as i64));
        self.schema.maximum = Some(RangeValue::I64(i16::max_value() as i64));
        self
    }

    pub fn i32(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::I64(i32::min_value() as i64));
        self.schema.maximum = Some(RangeValue::I64(i32::max_value() as i64));
        self
    }

    pub fn i64(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::I64(i64::min_value()));
        self.schema.maximum = Some(RangeValue::I64(i64::max_value()));
        self
    }

    pub fn isize(&mut self) -> &mut Self {
        //isize -> i64
        self.schema.minimum = Some(RangeValue::I64(i64::min_value()));
        self.schema.maximum = Some(RangeValue::I64(i64::max_value()));
        self
    }

    pub fn u8(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::U64(u8::min_value() as u64));
        self.schema.maximum = Some(RangeValue::U64(u8::max_value() as u64));
        self
    }

    pub fn u16(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::U64(u16::min_value() as u64));
        self.schema.maximum = Some(RangeValue::U64(u16::max_value() as u64));
        self
    }

    pub fn u32(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::U64(u32::min_value() as u64));
        self.schema.maximum = Some(RangeValue::U64(u32::max_value() as u64));
        self
    }

    pub fn u64(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::U64(u64::min_value()));
        self.schema.maximum = Some(RangeValue::U64(u64::max_value()));
        self
    }

    pub fn usize(&mut self) -> &mut Self {
        //usize -> u64
        self.schema.minimum = Some(RangeValue::U64(u64::min_value()));
        self.schema.maximum = Some(RangeValue::U64(u64::max_value()));
        self
    }
}

impl NumberBuilder {
    pub fn new() -> NumberBuilder {
        NumberBuilder {
            schema: JsonSchema::from_types(SchemaTypes::Number),
        }
    }

    pub fn build(&self) -> JsonSchema {
        self.schema.clone()
    }

    pub fn f32(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::F64(std::f32::MIN as f64));
        self.schema.maximum = Some(RangeValue::F64(std::f32::MAX as f64));
        self
    }

    pub fn f64(&mut self) -> &mut Self {
        self.schema.minimum = Some(RangeValue::F64(std::f64::MIN));
        self.schema.maximum = Some(RangeValue::F64(std::f64::MAX));
        self
    }
}

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
        match self.schema.additional_properties {
            Some(ref mut v) => {
                v.push(prop);
            }
            None => {
                self.schema.additional_properties = Some(vec![prop]);
            }
        }
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

    pub fn push(&mut self, prop: JsonSchema) {
        match self.schema.one_of {
            Some(ref mut v) => {
                v.push(prop);
            }
            None => {
                self.schema.one_of = Some(vec![prop]);
            }
        };
    }
}

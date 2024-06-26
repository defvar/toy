use crate::typed;
use crate::typed::AllowedTypes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toy_core::data::{Map, Value};
use toy_pack::Schema;

pub trait Transformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()>;
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Schema)]
pub enum NameOrIndexTransformer {
    Name(String),
    Index(u32),
}

impl Default for NameOrIndexTransformer {
    fn default() -> Self {
        NameOrIndexTransformer::Index(0)
    }
}

#[derive(Clone, Debug)]
pub struct MappingTransformer(pub Map<String, String>);

#[derive(Clone, Debug)]
pub struct NamingTransformer(pub HashMap<String, u32>);

#[derive(Clone, Debug)]
pub struct IndexingTransformer(pub Vec<String>);

#[derive(Clone, Debug)]
pub struct ReindexingTransformer(pub Vec<u32>);

#[derive(Clone, Debug)]
pub struct RenameTransformer(pub HashMap<String, String>);

#[derive(Clone, Debug)]
pub struct PutTransformer(pub HashMap<String, PutValueTransformer>);

#[derive(Clone, Debug)]
pub struct RemoveByNameTransformer(pub Vec<String>);

#[derive(Clone, Debug)]
pub struct RemoveByIndexTransformer(pub Vec<u32>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Schema)]
pub struct PutValueTransformer {
    value: Option<String>,
    tp: AllowedTypes,
}

#[derive(Clone, Debug)]
pub struct SingleValueTransformer(pub NameOrIndexTransformer);

#[derive(Clone, Debug)]
pub struct ToMapTransformer(pub String);

#[derive(Clone, Debug)]
pub struct ToSeqTransformer();

impl Transformer for MappingTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Map(_) => {
                let map = Map::with_capacity(self.0.len());
                let mut r = Value::from(map);
                for (k, v) in &self.0 {
                    r.insert_by_path(v, value.path(k).map_or(Value::None, |x| x.clone()));
                }
                *value = r;
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for NamingTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Seq(src) => {
                let map = Map::with_capacity(self.0.len());
                let mut r = Value::from(map);
                for (k, v) in &self.0 {
                    r.insert_by_path(k, src.get(*v as usize).map_or(Value::None, |x| x.clone()));
                }
                *value = r;
                Ok(())
            }
            ref v if self.0.keys().len() == 1 => {
                let map = Map::with_capacity(1);
                let mut r = Value::from(map);
                let name = self.0.keys().nth(0).unwrap();
                r.insert_by_path(name, (*v).clone());
                *value = r;
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for IndexingTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Map(_) => {
                let mut r = Vec::with_capacity(self.0.len());
                for k in &self.0 {
                    r.push(value.path(k.as_str()).map_or(Value::None, |x| x.clone()));
                }
                *value = Value::from(r);
                Ok(())
            }
            ref v if self.0.len() == 1 => {
                let mut r = Vec::with_capacity(1);
                r.push((*v).clone());
                *value = Value::from(r);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for ReindexingTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Seq(src) => {
                let mut r = Vec::with_capacity(self.0.len());
                for i in &self.0 {
                    r.push(src.get(*i as usize).map_or(Value::None, |x| x.clone()));
                }
                *value = Value::from(r);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for RenameTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Map(src) => {
                // keep original field ordering.

                let mut r = Map::with_capacity(src.len());
                for (k, v) in src {
                    let new_key = self.0.get(k).unwrap_or(k);
                    r.insert(new_key.clone(), v.clone());
                }
                *value = Value::from(r);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for PutTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Map(src) => {
                for (k, vt) in &self.0 {
                    src.insert(k.clone(), vt.value());
                }
                Ok(())
            }
            Value::Seq(src) => {
                for (_, vt) in &self.0 {
                    src.push(vt.value());
                }
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for RemoveByNameTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Map(src) => {
                for k in &self.0 {
                    src.remove(k.as_str());
                }
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for RemoveByIndexTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Seq(src) => {
                for i in &self.0 {
                    src.remove(*i as usize);
                }
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl PutValueTransformer {
    pub fn new(value: Option<String>, tp: AllowedTypes) -> PutValueTransformer {
        PutValueTransformer { value, tp }
    }

    pub fn value(&self) -> Value {
        let v = self
            .value
            .clone()
            .map(|x| Value::from(x))
            .unwrap_or(Value::None);
        typed::cast(&v, self.tp, self.value.as_ref().map(|x| x.as_str())).unwrap_or(Value::None)
    }
}

impl Transformer for SingleValueTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        let v = match value {
            Value::Map(_) => match &self.0 {
                NameOrIndexTransformer::Name(k) => Ok(value.path(&k)),
                _ => Err(Option::<&Value>::None),
            },
            Value::Seq(vec) => match &self.0 {
                NameOrIndexTransformer::Index(i) => Ok(vec.get(*i as usize)),
                _ => Err(Option::<&Value>::None),
            },
            _ => Err(Option::<&Value>::None),
        };
        match v {
            Ok(Some(v)) => {
                *value = v.clone();
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for ToMapTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        let mut map = Map::new();
        map.insert(self.0.clone(), value.clone());
        *value = Value::Map(map);
        Ok(())
    }
}

impl Transformer for ToSeqTransformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        let vec = vec![value.clone()];
        *value = Value::Seq(vec);
        Ok(())
    }
}

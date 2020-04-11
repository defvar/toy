use crate::typed;
use std::collections::HashMap;
use toy_core::data::{Map, Value};
use toy_pack_derive::*;

pub trait Transformer {
    fn transform(&self, value: &mut Value) -> Result<(), ()>;
}

#[derive(Debug)]
pub struct Named(pub HashMap<String, u32>);

#[derive(Debug)]
pub struct Indexed(pub Vec<String>);

#[derive(Debug)]
pub struct Reorder(pub Vec<u32>);

#[derive(Debug)]
pub struct Rename(pub HashMap<String, String>);

#[derive(Debug)]
pub struct Put(pub HashMap<String, PutValue>);

#[derive(Debug)]
pub struct RemoveByName(pub Vec<String>);

#[derive(Debug)]
pub struct RemoveByIndex(pub Vec<u32>);

#[derive(Debug, Clone, PartialEq, UnPack)]
pub struct PutValue {
    value: Option<String>,
    v: Value, // string
    tp: String,
}

impl Transformer for Named {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Seq(src) => {
                let mut r = Map::with_capacity(self.0.len());
                for (k, v) in &self.0 {
                    r.insert(
                        k.clone(),
                        src.get(*v as usize).map_or(Value::None, |x| x.clone()),
                    );
                }
                *value = Value::from(r);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Transformer for Indexed {
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
            _ => Err(()),
        }
    }
}

impl Transformer for Reorder {
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

impl Transformer for Rename {
    fn transform(&self, value: &mut Value) -> Result<(), ()> {
        match value {
            Value::Map(src) => {
                let mut r = Map::with_capacity(self.0.len());
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

impl Transformer for Put {
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

impl Transformer for RemoveByName {
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

impl Transformer for RemoveByIndex {
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

impl PutValue {
    pub fn new(value: Option<String>, tp: &str) -> PutValue {
        let v = value.clone().map(|x| Value::from(x)).unwrap_or(Value::None);
        PutValue {
            value,
            v,
            tp: tp.to_string(),
        }
    }

    pub fn value(&self) -> Value {
        typed::cast(&self.v, self.tp.as_str()).unwrap_or(Value::None)
    }
}

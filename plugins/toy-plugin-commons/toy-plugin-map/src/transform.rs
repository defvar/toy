use crate::typed;
use std::collections::HashMap;
use toy_core::data::{Map, Value};
use toy_pack_derive::*;

#[derive(Debug)]
pub enum Transform {
    Named(HashMap<String, u32>),
    Indexed(Vec<String>),
    Reorder(Vec<u32>),
    Put(HashMap<String, PutValue>),
    RemoveByName(Vec<String>),
    RemoveByIndex(Vec<u32>),
}

#[derive(Debug, Clone, PartialEq, UnPack)]
pub struct PutValue {
    value: Option<String>,
    v: Value, // string
    tp: String,
}

impl Transform {
    pub fn transform(&self, value: &mut Value) -> Option<Value> {
        match self {
            Transform::Named(named) => match value {
                Value::Seq(src) => {
                    let mut r = Map::new();
                    for (k, v) in named {
                        r.insert(
                            k.clone(),
                            src.get(*v as usize).map_or(Value::None, |x| x.clone()),
                        );
                    }
                    Some(Value::from(r))
                }
                _ => None,
            },
            Transform::Indexed(indexed) => match value {
                Value::Map(src) => {
                    let mut r = Vec::new();
                    for k in indexed {
                        r.push(src.get(k.as_str()).map_or(Value::None, |x| x.clone()));
                    }
                    Some(Value::from(r))
                }
                _ => None,
            },
            Transform::Reorder(reorder) => match value {
                Value::Seq(src) => {
                    let mut r = Vec::new();
                    for i in reorder {
                        r.push(src.get(*i as usize).map_or(Value::None, |x| x.clone()));
                    }
                    Some(Value::from(r))
                }
                _ => None,
            },
            Transform::Put(put) => match value {
                Value::Map(src) => {
                    for (k, vt) in put {
                        src.insert(k.clone(), vt.value());
                    }
                    Some(Value::from(src))
                }
                Value::Seq(src) => {
                    for (_, vt) in put {
                        src.push(vt.value());
                    }
                    Some(Value::from(src))
                }
                _ => None,
            },
            Transform::RemoveByName(remove_names) => match value {
                Value::Map(src) => {
                    for k in remove_names {
                        src.remove(k.as_str());
                    }
                    Some(Value::from(src))
                }
                _ => None,
            },
            Transform::RemoveByIndex(remove_idx) => match value {
                Value::Seq(src) => {
                    for i in remove_idx {
                        src.remove(*i as usize);
                    }
                    Some(Value::from(src))
                }
                _ => None,
            },
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

//! Model for rbac/role api.

use crate::common::KVObject;
use serde::{Deserialize, Serialize};

pub const RESOURCE_ALL: &'static str = "*";
pub const VERB_ALL: &'static str = "*";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    resources: Vec<String>,
    verbs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    name: String,
    note: Option<String>,
    rules: Vec<Rule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleList {
    items: Vec<Role>,
    count: u32,
}

impl Rule {
    pub fn new(resources: Vec<String>, verbs: Vec<String>) -> Self {
        Self { resources, verbs }
    }

    pub fn resources(&self) -> &Vec<String> {
        &self.resources
    }

    pub fn verbs(&self) -> &Vec<String> {
        &self.verbs
    }
}

impl Role {
    pub fn new<P: Into<String>>(name: P, note: Option<P>, rules: Vec<Rule>) -> Self {
        Self {
            name: name.into(),
            note: note.map(|x| x.into()),
            rules,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}

impl KVObject for Role {
    fn key(&self) -> &str {
        &self.name
    }
}

impl RoleList {
    pub fn new(items: Vec<Role>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

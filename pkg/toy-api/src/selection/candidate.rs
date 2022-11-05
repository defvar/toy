//!　Candidate values to apply the condition to.
//!　Struct to build candidate values from api's data model in order to exclude data that does not match the predicate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use toy_core::data::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Candidates {
    map: HashMap<String, CandidatePart>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidatePart {
    name: String,
    value: Value,
}

impl Candidates {
    pub fn new(candidates: &[CandidatePart]) -> Self {
        let map = candidates
            .iter()
            .map(|x| (x.name.clone(), x.clone()))
            .collect();
        Self { map }
    }

    pub fn empty() -> Self {
        Self {
            map: HashMap::with_capacity(0),
        }
    }

    pub fn with_candidate(mut self, name: impl Into<String>, value: Value) -> Self {
        let key = name.into();
        self.map.insert(key.clone(), CandidatePart::new(key, value));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&CandidatePart> {
        self.map.get(name)
    }
}

impl Default for Candidates {
    fn default() -> Self {
        Candidates::new(&[])
    }
}

impl CandidatePart {
    pub fn new(name: impl Into<String>, value: Value) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl PartialEq for CandidatePart {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for CandidatePart {}

impl Hash for CandidatePart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

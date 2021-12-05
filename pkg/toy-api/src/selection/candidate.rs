use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use toy_core::data::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidateMap {
    map: HashMap<String, Candidate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Candidate {
    name: String,
    value: Value,
}

impl CandidateMap {
    pub fn new(candidates: &[Candidate]) -> Self {
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
        self.map.insert(key.clone(), Candidate::new(key, value));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&Candidate> {
        self.map.get(name)
    }
}

impl Default for CandidateMap {
    fn default() -> Self {
        CandidateMap::new(&[])
    }
}

impl Candidate {
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

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Candidate {}

impl Hash for Candidate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

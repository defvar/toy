//! Apply the predicate to the struct's field to selection the necessary data.

use crate::common::SelectionCandidate;
use crate::selection::candidate::Candidate;
use crate::selection::Operator;
use serde::{Deserialize, Serialize};
use toy_core::data::Value;

/// Selection infomation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Selection {
    preds: Vec<Predicate>,
}

/// A single predicate to be selection. It consists of a field name, an operator, and a value.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Predicate {
    field: String,
    op: Operator,
    value: Value,
}

impl Selection {
    pub fn new(preds: Vec<Predicate>) -> Self {
        Self { preds }
    }

    pub fn empty() -> Self {
        Self {
            preds: Vec::with_capacity(0),
        }
    }

    pub fn preds(&self) -> &[Predicate] {
        &self.preds
    }

    pub fn is_match(&self, candidate: &impl SelectionCandidate) -> bool {
        let map = candidate.candidate_map();
        if map.is_empty() {
            return true;
        }
        self.preds.iter().all(|x| {
            let c = map.get(&x.field).map(|v| x.is_match(v));
            c.is_some() && c.unwrap()
        })
    }

    pub fn eq(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.preds.push(Predicate::new(field, Operator::Eq, value));
        self
    }

    pub fn not_eq(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.preds
            .push(Predicate::new(field, Operator::NotEq, value));
        self
    }

    pub fn greater_than(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.preds
            .push(Predicate::new(field.into(), Operator::GreaterThan, value));
        self
    }

    pub fn less_than(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.preds
            .push(Predicate::new(field, Operator::LessThan, value));
        self
    }

    pub fn contains(mut self, field: impl Into<String>, value: Option<impl Into<Value>>) -> Self {
        if value.is_some() {
            self.preds
                .push(Predicate::new(field, Operator::Contains, value.unwrap()));
        }
        self
    }
}

impl Default for Selection {
    fn default() -> Self {
        Selection::new(vec![])
    }
}

impl Predicate {
    pub fn new(field: impl Into<String>, op: Operator, value: impl Into<Value>) -> Self {
        Self {
            field: field.into(),
            op,
            value: value.into(),
        }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn op(&self) -> Operator {
        self.op
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn is_match(&self, candidate: &Candidate) -> bool {
        let l = &self.value;
        let r = candidate.value();
        match self.op {
            Operator::Eq => l == r,
            Operator::NotEq => l != r,
            Operator::LessThan => l > r,
            Operator::GreaterThan => l < r,
            Operator::Contains => {
                if l.is_string() && r.is_string() {
                    r.as_str().unwrap().contains(l.as_str().unwrap())
                } else {
                    false
                }
            }
        }
    }
}

//! Apply the predicate to the struct's field to selection the necessary data.

use crate::common::SelectionCandidate;
use crate::selection::candidate::CandidatePart;
use crate::selection::operator::operator_strings;
use crate::selection::Operator;
use regex::Regex;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use toy_core::data::Value;

/// Selection infomation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector {
    preds: Vec<Predicate>,
}

/// A single predicate to be selection. It consists of a field name, an operator, and a value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicate {
    field: String,
    op: Operator,
    value: Value,
}

impl Selector {
    pub fn new(preds: Vec<Predicate>) -> Self {
        Self { preds }
    }

    pub fn empty() -> Self {
        Self {
            preds: Vec::with_capacity(0),
        }
    }

    fn predicate_fields(&self) -> Vec<&str> {
        self.preds.iter().map(|x| x.field.as_str()).collect()
    }

    pub fn validation_fields<T: SelectionCandidate>(&self) -> Result<(), Vec<String>> {
        let unknowns = self
            .predicate_fields()
            .into_iter()
            .filter(|x| !T::candidate_fields().contains(x))
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        if !unknowns.is_empty() {
            Err(unknowns)
        } else {
            Ok(())
        }
    }

    pub fn validation_fields_by_names<'a>(&self, fileds: &'a [&'a str]) -> Result<(), Vec<String>> {
        let unknowns = self
            .predicate_fields()
            .into_iter()
            .filter(|x| !fileds.contains(x))
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        if !unknowns.is_empty() {
            Err(unknowns)
        } else {
            Ok(())
        }
    }

    pub fn preds(&self) -> &[Predicate] {
        &self.preds
    }

    /// Check candidate fields.
    ///
    /// # Example
    /// ```
    /// use toy_api::selection::candidate::CandidatePart;
    /// use toy_api::selection::Operator;
    /// use toy_api::selection::selector::Predicate;
    /// use toy_core::data::Value;
    ///
    /// let p = Predicate::new("name", Operator::Eq, "hoge");
    /// let cp = CandidatePart::new("name", "fuga");
    /// let result = p.is_match(&cp);
    /// assert_eq!(result, Ok(false));
    /// ```
    ///
    pub fn is_match(&self, candidate: &impl SelectionCandidate) -> Result<bool, String> {
        let map = candidate.candidates();
        if map.is_empty() {
            return Ok(true);
        }

        for pred in &self.preds {
            let c = map.get(&pred.field).map(|v| pred.is_match(v));
            match c {
                Some(Ok(true)) => (),
                Some(Ok(false)) => return Ok(false),
                _ => return Err(pred.field.clone()),
            };
        }
        Ok(true)
    }

    pub fn get(&self, field: &str) -> Option<&Predicate> {
        self.preds.iter().filter(|x| x.field == field).nth(0)
    }

    pub fn add(
        mut self,
        field: impl Into<String>,
        op: Operator,
        value: Option<impl Into<Value>>,
    ) -> Self {
        if value.is_some() {
            self.preds.push(Predicate::new(field, op, value.unwrap()));
        }
        self
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
}

impl Default for Selector {
    fn default() -> Self {
        Selector::new(vec![])
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

    pub fn is_match(&self, candidate: &CandidatePart) -> Result<bool, ()> {
        let mut l = &self.value;
        let r = candidate.value();
        let mut same_type_left_value = None;

        if !l.is_same_type(r) {
            match l.as_same_type(&r) {
                Some(v) => same_type_left_value = Some(v),
                None => return Err(()), //type error
            }
        }

        match same_type_left_value {
            Some(ref v) => l = v,
            None => (),
        };

        match self.op {
            Operator::Eq => Ok(l == r),
            Operator::NotEq => Ok(l != r),
            Operator::LessThan => Ok(l > r),
            Operator::LessThanOrEqual => Ok(l >= r),
            Operator::GreaterThan => Ok(l < r),
            Operator::GreaterThanOrEqual => Ok(l <= r),
            Operator::Match => {
                if l.is_string() && r.is_string() {
                    if let Ok(reg) = Regex::new(l.as_str().unwrap()) {
                        Ok(reg.is_match(r.as_str().unwrap()))
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }
            Operator::Unmatch => {
                if l.is_string() && r.is_string() {
                    if let Ok(reg) = Regex::new(l.as_str().unwrap()) {
                        Ok(!reg.is_match(r.as_str().unwrap()))
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn from_str(v: &str) -> Result<Predicate, ()> {
        let mut left: Option<&str> = None;
        let mut op: Result<Operator, ()> = Err(());
        let mut right: Option<&str> = None;

        for t in operator_strings() {
            if v.contains(t) {
                let vec: Vec<&str> = v.split(t).collect();
                left = vec.get(0).map(|x| x.trim());
                op = Operator::try_from(t);
                right = vec.get(1).map(|x| x.trim());
                break;
            }
        }

        match (left, op, right) {
            (Some(l), Ok(o), Some(r)) => Ok(Predicate {
                field: l.to_string(),
                op: o,
                value: Value::from(r),
            }),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        let qs = format!(
            "{}{}{}",
            &self.field,
            self.op.as_str(),
            self.value.as_str().unwrap_or("")
        );
        qs
    }
}

impl Serialize for Predicate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Predicate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PredicateVisitor)
    }
}

struct PredicateVisitor;

impl<'de> Visitor<'de> for PredicateVisitor {
    type Value = Predicate;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a predicate")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match Predicate::from_str(v) {
            Ok(p) => Ok(p),
            Err(()) => Err(E::custom(format!("invalid predicate: {}", v))),
        }
    }
}

impl Serialize for Selector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let qs = self
            .preds
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        serializer.serialize_str(&qs)
    }
}

impl<'de> Deserialize<'de> for Selector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SelectionVisitor)
    }
}

struct SelectionVisitor;

impl<'de> Visitor<'de> for SelectionVisitor {
    type Value = Selector;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a selection")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let vec: Vec<&str> = v.split(",").collect();
        let mut r = Selector::empty();
        for part in vec.iter().filter(|x| !x.is_empty()) {
            match Predicate::from_str(part) {
                Ok(p) => {
                    r = r.add(p.field, p.op, Some(p.value));
                }
                Err(()) => return Err(E::custom(format!("invalid predicate: {}", part))),
            }
        }
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Format, Indent};
    use crate::selection::candidate::CandidatePart;
    use crate::selection::selector::{Predicate, Selector};
    use crate::selection::Operator;
    use crate::services::ServiceSpecListOption;
    use serde::{Deserialize, Serialize};
    use toy_core::data::Value;

    #[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
    struct TestPredicateHolder {
        p: Predicate,
    }

    #[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
    struct TestSelection {
        s: Selector,
    }

    #[test]
    fn predicate_match() {
        let p = Predicate {
            field: "hoge".to_string(),
            op: Operator::Match,
            value: Value::from("^abc"),
        };
        let cp = CandidatePart::new("hoge", Value::String("abcdef".to_string()));
        let r = p.is_match(&cp);
        assert_eq!(r, Ok(true));

        let cp = CandidatePart::new("hoge", Value::String("fedcba".to_string()));
        let r = p.is_match(&cp);
        assert_eq!(r, Ok(false));
    }

    #[test]
    fn predicate_unmatch() {
        let p = Predicate {
            field: "hoge".to_string(),
            op: Operator::Unmatch,
            value: Value::from("^abc"),
        };
        let cp = CandidatePart::new("hoge", Value::String("fedcba".to_string()));
        let r = p.is_match(&cp);
        assert_eq!(r, Ok(true));

        let cp = CandidatePart::new("hoge", Value::String("abcdef".to_string()));
        let r = p.is_match(&cp);
        assert_eq!(r, Ok(false));
    }

    #[test]
    fn deser_selection() {
        let t: TestSelection =
            toy_pack_json::unpack("{ \"s\": \"hoge=abc,moge!=123\" } ".as_bytes()).unwrap();

        let vec = vec![
            Predicate {
                field: "hoge".to_string(),
                op: Operator::Eq,
                value: Value::from("abc"),
            },
            Predicate {
                field: "moge".to_string(),
                op: Operator::NotEq,
                value: Value::from("123"),
            },
        ];
        assert_eq!(t.s, Selector::new(vec));
    }

    #[test]
    fn deser_predicate_eq() {
        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge=abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::Eq,
                value: Value::from("abc")
            }
        );

        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge==abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::Eq,
                value: Value::from("abc")
            }
        );
    }

    #[test]
    fn deser_predicate_not_eq() {
        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge!=abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::NotEq,
                value: Value::from("abc")
            }
        );

        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge!==abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::NotEq,
                value: Value::from("abc")
            }
        );
    }

    #[test]
    fn deser_predicate_less() {
        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge<abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::LessThan,
                value: Value::from("abc")
            }
        );

        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge<=abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::LessThanOrEqual,
                value: Value::from("abc")
            }
        );
    }

    #[test]
    fn deser_predicate_greater() {
        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge>abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::GreaterThan,
                value: Value::from("abc")
            }
        );

        let t: TestPredicateHolder =
            toy_pack_json::unpack("{ \"p\": \"hoge>=abc\" } ".as_bytes()).unwrap();
        assert_eq!(
            t.p,
            Predicate {
                field: "hoge".to_string(),
                op: Operator::GreaterThanOrEqual,
                value: Value::from("abc")
            }
        );
    }
}

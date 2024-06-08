use std::fmt::{Display, Formatter};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use toy_core::data::Value;
use toy_pack::Schema;

#[derive(Debug, Clone, Eq, PartialEq, Schema)]
pub struct Predicate {
    field: String,
    op: Operator,
    value: String,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Serialize, Schema)]
pub enum Operator {
    /// =
    Eq,
    /// !=
    NotEq,
    /// candidate \> predicate value
    GreaterThan,
    /// candidate \>= predicate value
    GreaterThanOrEqual,
    /// candidate \< predicate value
    LessThan,
    /// candidate \<= predicate value
    LessThanOrEqual,
    /// Regex match. supported by string only.
    Match,
    /// Regex unmatch. supported by string only.
    Unmatch,
}

pub const fn operator_strings() -> &'static [&'static str] {
    &["!==", "!=", ">=", ">", "<=", "<", "==", "=~", "!~", "="]
}

impl Predicate {
    pub fn new(field: impl Into<String>, op: Operator, value: impl Into<String>) -> Self {
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

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_match(&self, candidate: &Value) -> bool {
        let l_opt = Value::from(&self.value).as_same_type(candidate);
        let r = candidate;

        if l_opt.is_none() {
            return false;
        }

        let l = &l_opt.unwrap();

        match self.op {
            Operator::Eq => l == r,
            Operator::NotEq => l != r,
            Operator::LessThan => l > r,
            Operator::LessThanOrEqual => l >= r,
            Operator::GreaterThan => l < r,
            Operator::GreaterThanOrEqual => l <= r,
            Operator::Match => {
                if l.is_string() && r.is_string() {
                    if let Ok(reg) = Regex::new(l.as_str().unwrap()) {
                        reg.is_match(r.as_str().unwrap())
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Operator::Unmatch => {
                if l.is_string() && r.is_string() {
                    if let Ok(reg) = Regex::new(l.as_str().unwrap()) {
                        !reg.is_match(r.as_str().unwrap())
                    } else {
                        false
                    }
                } else {
                    false
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
                value: r.to_string(),
            }),
            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        let qs = format!(
            "{}{}{}",
            &self.field,
            self.op.as_str(),
            &self.value
        );
        qs
    }
}

impl Display for Predicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.field, self.op, self.value)
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

impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "=" | "==" => Ok(Operator::Eq),
            "!=" | "!==" => Ok(Operator::NotEq),
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterThanOrEqual),
            "<" => Ok(Operator::LessThan),
            "<=" => Ok(Operator::LessThanOrEqual),
            "=~" => Ok(Operator::Match),
            "!~" => Ok(Operator::Unmatch),
            _ => Err(()),
        }
    }
}

impl TryFrom<&&str> for Operator {
    type Error = ();

    fn try_from(value: &&str) -> Result<Self, Self::Error> {
        TryFrom::try_from(*value)
    }
}

impl Operator {
    pub fn as_str(&self) -> &str {
        match self {
            Operator::Eq => "==",
            Operator::NotEq => "!=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThan => "<",
            Operator::LessThanOrEqual => "<=",
            Operator::Match => "=~",
            Operator::Unmatch => "!~",
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(OperatorVisitor)
    }
}

struct OperatorVisitor;

impl<'de> Visitor<'de> for OperatorVisitor {
    type Value = Operator;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("a operator [{:?}].", operator_strings()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
    {
        let op = Operator::try_from(v);
        if op.is_err() {
            Err(E::custom(format!("unknown operator: {}", v)))
        } else {
            Ok(op.unwrap())
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(e) => Err(E::custom(e)),
        }
    }
}

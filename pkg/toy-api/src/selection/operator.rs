//! Operator of a predicate.
//!

use serde::de::Error;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;

/// Operator of a predicate.
#[derive(Clone, Debug, Copy, Eq, PartialEq, Serialize)]
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
        match *value {
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

#[cfg(test)]
mod tests {
    use crate::selection::Operator;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
    struct Test {
        ope: Operator,
    }

    #[test]
    fn deser_operator_eq() {
        let t: Test = toy_pack_json::unpack("{ \"ope\": \"=\" } ".as_bytes()).unwrap();
        assert_eq!(t.ope, Operator::Eq);

        let t: Test = toy_pack_json::unpack("{ \"ope\": \"==\" } ".as_bytes()).unwrap();
        assert_eq!(t.ope, Operator::Eq);
    }

    #[test]
    fn deser_operator_not_eq() {
        let t: Test = toy_pack_json::unpack("{ \"ope\": \"!=\" } ".as_bytes()).unwrap();
        assert_eq!(t.ope, Operator::NotEq);

        let t: Test = toy_pack_json::unpack("{ \"ope\": \"!==\" } ".as_bytes()).unwrap();
        assert_eq!(t.ope, Operator::NotEq);
    }

    #[test]
    fn deser_operator_less() {
        let t: Test = toy_pack_json::unpack("{ \"ope\": \"<\" } ".as_bytes()).unwrap();
        assert_eq!(t.ope, Operator::LessThan);

        let t: Test = toy_pack_json::unpack("{ \"ope\": \"<=\" } ".as_bytes()).unwrap();
        assert_eq!(t.ope, Operator::LessThanOrEqual);
    }
}

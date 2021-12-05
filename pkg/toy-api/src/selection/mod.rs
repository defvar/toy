use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Operator {
    /// =
    Eq,
    /// !=
    NotEq,
    /// candidate \> predicate value
    GreaterThan,
    /// candidate \< predicate value
    LessThan,
    /// candidate contains predicate value. supportted by string only.
    Contains,
}

pub mod field;

pub mod candidate;

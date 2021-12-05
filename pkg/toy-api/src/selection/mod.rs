//! It provides the ability to apply conditions to struct fields and filter
//! them in order to select the required data on the application side.

use serde::{Deserialize, Serialize};

/// Operator of a predicate.
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

pub mod candidate;
pub mod field;

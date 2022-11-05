//! It provides the ability to apply conditions to struct fields and filter
//! them in order to select the required data on the application side.

pub mod candidate;
pub mod fields;
pub mod operator;
pub mod selector;
pub use operator::Operator;

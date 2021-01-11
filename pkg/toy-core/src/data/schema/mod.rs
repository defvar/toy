//! Create Json Schema from Graph setting information.
//!
//! ```
//! use toy_pack::{schema::to_schema, Schema};
//! use toy_core::data::schema::visitors::*;
//!
//! #[derive(Schema)]
//! struct Config {
//!   capacity: usize,
//!   path: String,
//! }
//!
//! let json_schema = to_schema::<Config, JsonSchemaVisitor>("config", JsonSchemaVisitor).unwrap();
//!
//! ```

pub mod builders;
mod error;
mod json_schema;
pub mod visitors;

pub use error::SchemaScanError;
pub use json_schema::{JsonSchema, SchemaTypes};

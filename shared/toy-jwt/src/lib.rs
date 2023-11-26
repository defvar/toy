//! # toy JWT Library

#![feature(error_generic_member_access)]

mod algorithm;
pub mod error;
mod header;

pub mod decode;
pub mod encode;
mod validation;

pub use algorithm::Algorithm;
pub use header::Header;
pub use validation::Validation;

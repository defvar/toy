//! Implementation for YAML

#[macro_use]
extern crate failure;

pub use self::decoder::{
    Decoder,
    Event,
};

pub mod error;
mod decoder;
mod deserializer;
mod deser_ops;

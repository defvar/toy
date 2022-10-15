#![feature(error_generic_member_access, provide_any, type_alias_impl_trait)]

mod error;
mod function;

pub mod config {
    pub use super::function::LuaFunctionConfig;
}

pub mod service {
    pub use super::function::{LuaFunction, LuaFunctionContext};
}

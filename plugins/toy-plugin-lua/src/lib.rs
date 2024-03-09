#![feature(
error_generic_member_access,
type_alias_impl_trait,
impl_trait_in_assoc_type
)]

mod error;
mod function;
mod plugin;

pub use plugin::lua;

pub mod config {
    pub use super::function::LuaFunctionConfig;
}

pub mod service {
    pub use super::function::{LuaFunction, LuaFunctionContext};
}

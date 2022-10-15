#![feature(error_generic_member_access, provide_any)]

mod claims;
mod constants;
mod credential;
pub mod error;
pub mod firebase;
mod jwk;
mod scope;
mod token;

pub use self::claims::Claims;
pub use self::scope::Scope;
pub use self::token::{request_token, GToken, GTokenError};

use std::sync::Arc;
use toy_core::registry::ServiceSet;

mod filters;
mod handlers;

pub type ServiceRegistry<S, F> = Arc<ServiceSet<S, F>>;

pub use filters::services;

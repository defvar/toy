//! common module for api.

pub mod body;
pub mod error;
pub mod filter;
pub mod handler;
pub mod query;
pub mod reply;
pub mod validator;

pub mod constants {
    use toy_core::task::TaskId;

    pub static GRAPHS_KEY_PREFIX: &'static str = "/toy/graphs";
    pub static SUPERVISORS_KEY_PREFIX: &'static str = "/toy/supervisors";
    pub static PENDINGS_KEY_PREFIX: &'static str = "/toy/pendings";
    pub static SERVICES_KEY_PREFIX: &'static str = "/toy/services";
    pub static ROLE_KEY_PREFIX: &'static str = "/toy/roles";
    pub static ROLE_BINDING_KEY_PREFIX: &'static str = "/toy/roleBindings";
    pub static SECRET_KEY_PREFIX: &'static str = "/toy/secrets";

    pub fn pending_key(id: TaskId) -> String {
        format!("{}/{}", PENDINGS_KEY_PREFIX, id)
    }

    pub(crate) fn generate_key(prefix: impl AsRef<str>, key: impl AsRef<str>) -> String {
        format!("{}/{}", prefix.as_ref(), key.as_ref())
    }
}

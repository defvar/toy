//! common module for api.

pub mod body;
pub mod error;
pub mod query;
pub mod reply;

pub mod constants {
    use toy_core::task::TaskId;

    pub static GRAPHS_KEY_PREFIX: &'static str = "/toy/graphs";
    pub static SUPERVISORS_KEY_PREFIX: &'static str = "/toy/supervisors";
    pub static PENDINGS_KEY_PREFIX: &'static str = "/toy/pendings";
    pub static SERVICES_KEY_PREFIX: &'static str = "/toy/services";

    pub fn graph_key(part: String) -> String {
        format!("{}/{}", GRAPHS_KEY_PREFIX, part)
    }

    pub fn pending_key(id: TaskId) -> String {
        format!("{}/{}", PENDINGS_KEY_PREFIX, id)
    }

    pub fn supervisor_key(name: String) -> String {
        format!("{}/{}", SUPERVISORS_KEY_PREFIX, name)
    }

    pub fn service_key(name: String) -> String {
        format!("{}/{}", SERVICES_KEY_PREFIX, name)
    }
}

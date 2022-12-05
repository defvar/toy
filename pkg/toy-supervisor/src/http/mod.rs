mod handler;
mod server;

use serde::Serialize;
pub use server::Server;
use toy_core::task::TaskId;

#[derive(Serialize)]
pub struct Status {
    pub(crate) name: String,
    // rfc3339
    pub(crate) started_at: Option<String>,
    pub(crate) running_tasks: Vec<(TaskId, String)>,
}

#[derive(Serialize)]
pub struct Metrics {
    pub(crate) name: String,
    pub(crate) task_start_count: u64,
}

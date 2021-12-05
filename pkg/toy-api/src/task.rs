//! Model for task api.

use crate::common::{Format, ListOption, ListOptionLike};
use crate::graph::Graph;
use crate::selection::field::Selection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toy_core::prelude::TaskId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PendingStatus {
    Created,
    Allocated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTask {
    task_id: TaskId,
    status: PendingStatus,
    allocated_supervisor: Option<String>,
    allocated_at: Option<String>,
    graph: Option<Graph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTaskList {
    items: Vec<PendingTask>,
    count: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PendingResult {
    task_id: TaskId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateRequest {
    supervisor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateResponse {
    task_id: TaskId,
    status: AllocateStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllocateStatus {
    Ok,
    NotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskLog {
    task_id: TaskId,
    payload: Vec<TaskLogInner>,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskLogInner {
    message: String,
    target: String,
    graph: String,
    uri: Option<String>,
    level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<TasksInner>,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TasksInner {
    task_id: TaskId,
    graph: String,
    operation: String,
    operation_at: DateTime<Utc>,
    supervisor: String,
}

impl Default for PendingStatus {
    fn default() -> Self {
        PendingStatus::Created
    }
}

impl PendingTask {
    pub fn new(task_id: TaskId, graph: Graph) -> Self {
        Self {
            task_id,
            status: PendingStatus::Created,
            allocated_supervisor: None,
            allocated_at: None,
            graph: Some(graph),
        }
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn status(&self) -> &PendingStatus {
        &self.status
    }

    pub fn graph(&self) -> Option<&Graph> {
        self.graph.as_ref()
    }

    pub fn allocate<S: Into<String>, T: Into<String>>(self, name: S, allocated_at: T) -> Self {
        Self {
            task_id: self.task_id,
            graph: self.graph,
            status: PendingStatus::Allocated,
            allocated_supervisor: Some(name.into()),
            allocated_at: Some(allocated_at.into()),
        }
    }
}

impl PendingTaskList {
    pub fn new(pendings: Vec<PendingTask>) -> Self {
        let count = pendings.len() as u32;
        Self {
            items: pendings,
            count,
        }
    }

    pub fn pendings(&self) -> &Vec<PendingTask> {
        &self.items
    }
}

impl PendingResult {
    pub fn from_id(id: TaskId) -> Self {
        Self { task_id: id }
    }
}

impl AllocateRequest {
    pub fn new(supervisor: impl Into<String>) -> Self {
        Self {
            supervisor: supervisor.into(),
        }
    }

    pub fn supervisor(&self) -> &str {
        &self.supervisor
    }
}

impl AllocateResponse {
    pub fn ok(id: TaskId) -> Self {
        Self {
            task_id: id,
            status: AllocateStatus::Ok,
        }
    }

    pub fn not_found(id: TaskId) -> Self {
        Self {
            task_id: id,
            status: AllocateStatus::NotFound,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == AllocateStatus::Ok
    }
}

impl Default for AllocateStatus {
    fn default() -> Self {
        AllocateStatus::NotFound
    }
}

impl TaskLog {
    pub fn new(task_id: TaskId, payload: Vec<TaskLogInner>) -> Self {
        let count = payload.len() as u32;
        Self {
            task_id,
            payload,
            count,
        }
    }
}

impl TaskLogInner {
    pub fn new<S: Into<String>>(message: S, target: S, graph: S, uri: Option<S>, level: S) -> Self {
        Self {
            message: message.into(),
            target: target.into(),
            graph: graph.into(),
            uri: uri.map(Into::into),
            level: level.into(),
        }
    }
}

impl Tasks {
    pub fn new(tasks: Vec<TasksInner>) -> Self {
        let count = tasks.len() as u32;
        Self { tasks, count }
    }
}

impl TasksInner {
    pub fn new<T: AsRef<str>, S: Into<String>>(
        task_id: T,
        operation: S,
        operation_at: DateTime<Utc>,
        graph: S,
        supervisor: S,
    ) -> Result<Self, ()> {
        let id = match TaskId::parse_str(task_id.as_ref()) {
            Ok(id) => id,
            Err(_) => return Err(()),
        };
        Ok(Self {
            task_id: id,
            operation: operation.into(),
            operation_at,
            graph: graph.into(),
            supervisor: supervisor.into(),
        })
    }
}

/// Watch api option.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WatchOption {
    format: Option<Format>,
}

impl WatchOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// Post api option.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostOption {
    format: Option<Format>,
}

impl PostOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// Allocate api option.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllocateOption {
    format: Option<Format>,
}

impl AllocateOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

/// Log api option.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogOption {
    format: Option<Format>,
}

impl LogOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskListOption {
    #[serde(flatten)]
    common: ListOption,

    #[serde(with = "crate::common::format::rfc3399_option")]
    timestamp: Option<DateTime<Utc>>,
}

impl TaskListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
            timestamp: None,
        }
    }

    pub fn timestamp(&self) -> Option<&DateTime<Utc>> {
        self.timestamp.as_ref()
    }
}

impl ListOptionLike for TaskListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }

    fn selection(&self) -> Selection {
        Selection::empty()
    }
}

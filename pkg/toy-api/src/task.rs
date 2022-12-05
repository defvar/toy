//! Model for task api.

use crate::common::{Format, ListObject, ListOption, ListOptionLike};
use crate::graph::Graph;
use crate::supervisors::SupervisorName;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toy_core::prelude::TaskId;
use toy_core::Uri;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PendingStatus {
    Created,
    Allocated,
    AllocateFailed,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTask {
    task_id: TaskId,
    status: PendingStatus,
    allocated_supervisor: Option<SupervisorName>,
    allocated_on: Option<DateTime<Utc>>,
    graph: Graph,
    created_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTaskList {
    items: Vec<PendingTask>,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingResult {
    task_id: TaskId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateRequest {
    supervisor: SupervisorName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateResponse {
    task_id: TaskId,
    status: AllocateStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllocateStatus {
    Ok,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinishResponse {
    Ok { id: TaskId },
    NotFound { id: TaskId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEventList {
    items: Vec<TaskEvent>,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    task_id: TaskId,
    name: String,
    uri: Uri,
    event: String,
    timestamp: DateTime<Utc>,
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
            allocated_on: None,
            graph,
            created_on: Utc::now(),
        }
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn status(&self) -> PendingStatus {
        self.status
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn allocate<S: Into<SupervisorName>>(self, name: S, allocated_at: DateTime<Utc>) -> Self {
        Self {
            status: PendingStatus::Allocated,
            allocated_supervisor: Some(name.into()),
            allocated_on: Some(allocated_at),
            ..self
        }
    }

    pub fn allocate_failed<S: Into<SupervisorName>>(
        self,
        name: S,
        allocated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            status: PendingStatus::AllocateFailed,
            allocated_supervisor: Some(name.into()),
            allocated_on: Some(allocated_at),
            ..self
        }
    }

    pub fn finished(self, _finished_at: DateTime<Utc>) -> Self {
        Self {
            status: PendingStatus::Finished,
            ..self
        }
    }

    pub fn is_dispatchable(&self) -> bool {
        match self.status {
            PendingStatus::Created | PendingStatus::AllocateFailed => true,
            _ => false,
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

    pub fn none(id: TaskId) -> Self {
        Self {
            task_id: id,
            status: AllocateStatus::None,
        }
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn is_ok(&self) -> bool {
        self.status == AllocateStatus::Ok
    }
}

impl FinishResponse {
    pub fn ok(id: TaskId) -> Self {
        FinishResponse::Ok { id }
    }

    pub fn not_found(id: TaskId) -> Self {
        FinishResponse::NotFound { id }
    }
}

impl Default for AllocateStatus {
    fn default() -> Self {
        AllocateStatus::None
    }
}

impl TaskEventList {
    pub fn new(items: Vec<TaskEvent>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

impl TaskEvent {
    pub fn new<S: Into<String>>(
        task_id: TaskId,
        name: S,
        uri: Uri,
        event: S,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            task_id,
            name: name.into(),
            uri,
            event: event.into(),
            timestamp,
        }
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
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

impl ListObject<TasksInner> for Tasks {
    fn items(&self) -> &[TasksInner] {
        &self.tasks
    }

    fn count(&self) -> u32 {
        self.count
    }
}

//////////////////////////////////
// Option
//////////////////////////////////

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

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp.clone()
    }
}

impl ListOptionLike for TaskListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }
}

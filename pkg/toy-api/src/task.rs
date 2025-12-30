//! Model for task api.

use crate::actors::ActorName;
use crate::common::{Format, ListObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::graph::Graph;
use crate::selection::candidate::Candidates;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toy_core::metrics::EventId;
use toy_core::prelude::TaskId;
use toy_core::{ServiceType, Uri};

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
    allocated_actor: Option<ActorName>,
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
    actor: ActorName,
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
    event_id: EventId,
    task_id: TaskId,
    name: String,
    service_type: ServiceType,
    uri: Uri,
    event: String,
    actor: String,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    items: Vec<Task>,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    task_id: TaskId,
    name: String,
    actor: String,
    event: String,
    timestamp: DateTime<Utc>,
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
            allocated_actor: None,
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

    pub fn allocate<S: Into<ActorName>>(self, name: S, allocated_at: DateTime<Utc>) -> Self {
        Self {
            status: PendingStatus::Allocated,
            allocated_actor: Some(name.into()),
            allocated_on: Some(allocated_at),
            ..self
        }
    }

    pub fn allocate_failed<S: Into<ActorName>>(self, name: S, allocated_at: DateTime<Utc>) -> Self {
        Self {
            status: PendingStatus::AllocateFailed,
            allocated_actor: Some(name.into()),
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
    pub fn new(actor: impl Into<String>) -> Self {
        Self {
            actor: actor.into(),
        }
    }

    pub fn actor(&self) -> &str {
        &self.actor
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

impl ListObject<TaskEvent> for TaskEventList {
    fn items(&self) -> &[TaskEvent] {
        &self.items
    }

    fn count(&self) -> u32 {
        self.count
    }
}

impl TaskEvent {
    pub fn new<S: Into<String>>(
        event_id: EventId,
        task_id: TaskId,
        name: S,
        service_type: ServiceType,
        uri: Uri,
        event: S,
        actor: S,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            event_id,
            task_id,
            name: name.into(),
            service_type,
            uri,
            event: event.into(),
            actor: actor.into(),
            timestamp,
        }
    }

    pub fn event_id(&self) -> EventId {
        self.event_id
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.service_type
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn actor(&self) -> &str {
        &self.actor
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}

impl SelectionCandidate for TaskEvent {
    fn candidate_fields() -> &'static [&'static str] {
        &["name", "service_type", "uri", "actor", "timestamp"]
    }

    fn candidates(&self) -> Candidates {
        Candidates::default()
            .with_candidate("name", self.name.to_string())
            .with_candidate("timestamp", self.timestamp)
    }
}

impl TaskList {
    pub fn new(items: Vec<Task>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

impl ListObject<Task> for TaskList {
    fn items(&self) -> &[Task] {
        &self.items
    }

    fn count(&self) -> u32 {
        self.count
    }
}

impl Task {
    pub fn new<T: AsRef<str>, S: Into<String>>(
        task_id: T,
        name: S,
        actor: S,
        event: S,
        timestamp: DateTime<Utc>,
    ) -> Result<Self, ()> {
        let id = match TaskId::parse_str(task_id.as_ref()) {
            Ok(id) => id,
            Err(_) => return Err(()),
        };
        Ok(Self {
            task_id: id,
            name: name.into(),
            actor: actor.into(),
            event: event.into(),
            timestamp,
        })
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn actor(&self) -> &str {
        &self.actor
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
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
pub struct TaskEventListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl TaskEventListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for TaskEventListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl TaskListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for TaskListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }
}

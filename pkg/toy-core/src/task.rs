//! A task is an workflow created based on graph.
//!

use crate::graph::Graph;
use crate::metrics::{EventRecord, MetricsEventKind, MetricsEvents};
use crate::{ServiceType, Uri};
use chrono::Utc;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Task Identifier
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskId {
    id: Uuid,
}

/// Context of Running Task.
/// You do **not** have to wrap the `TaskContext` it in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
#[derive(Clone)]
pub struct TaskContext {
    inner: Arc<Inner>,
    uri: Uri,
    current_span: Option<tracing::Span>,
    events: Arc<Mutex<MetricsEvents>>,
}

struct Inner {
    id: TaskId,
    started_at: SystemTime,
    graph: Graph,
}

impl TaskId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from(uuid: Uuid) -> Self {
        Self { id: uuid }
    }

    pub fn parse_str<T: AsRef<str>>(uuid: T) -> Result<TaskId, ()> {
        Uuid::parse_str(uuid.as_ref())
            .map(|id| TaskId::from(id))
            .map_err(|_| ())
    }

    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf str {
        self.id.as_hyphenated().encode_lower(buffer)
    }

    pub const fn encode_buffer() -> [u8; 45] {
        [0; 45]
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl fmt::Debug for TaskId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl Default for TaskId {
    fn default() -> Self {
        TaskId::from(Uuid::nil())
    }
}

impl FromStr for TaskId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TaskId::parse_str(s)
    }
}

impl AsRef<TaskId> for TaskId {
    fn as_ref(&self) -> &TaskId {
        self
    }
}

impl PartialEq<&TaskId> for TaskId {
    fn eq(&self, other: &&TaskId) -> bool {
        self.id == other.id
    }
}

macro_rules! task_span {
    ($name: ident, $level: ident) => {
        pub fn $name(&self) -> tracing::span::Span {
            tracing::span!(tracing::Level::$level, "Task", task=%self.id(), graph=%self.name(), uri=%self.uri())
        }
    };
}

impl TaskContext {
    pub fn new(id: TaskId, graph: Graph) -> Self {
        let task_name = graph.name().to_owned();
        Self {
            inner: Arc::new(Inner {
                id,
                started_at: SystemTime::now(),
                graph,
            }),
            uri: Uri::from(task_name),
            current_span: None,
            events: Arc::new(Mutex::new(MetricsEvents::new())),
        }
    }

    pub fn with_parts(id: TaskId, graph: Graph, events: Arc<Mutex<MetricsEvents>>) -> Self {
        let task_name = graph.name().to_owned();
        Self {
            inner: Arc::new(Inner {
                id,
                started_at: SystemTime::now(),
                graph,
            }),
            uri: Uri::from(task_name),
            current_span: None,
            events,
        }
    }

    pub fn with_uri(self, uri: &Uri) -> Self {
        let task_name = self.name().to_owned();
        Self {
            inner: self.inner,
            uri: Uri::from(format!("{}/{}", task_name, uri.clone())),
            current_span: self.current_span,
            events: self.events,
        }
    }

    pub fn id(&self) -> TaskId {
        self.inner.id
    }

    pub fn started_at(&self) -> SystemTime {
        self.inner.started_at
    }

    pub fn graph(&self) -> &Graph {
        &self.inner.graph
    }

    pub fn name(&self) -> &str {
        self.inner.graph.name()
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Get current span.
    pub fn span(&self) -> &tracing::Span {
        assert!(self.current_span.is_some(), "illegal task span.");
        self.current_span.as_ref().unwrap()
    }

    pub fn set_span(&mut self, span: tracing::Span) {
        self.current_span = Some(span);
    }

    task_span!(debug_span, DEBUG);
    task_span!(info_span, INFO);

    pub async fn push_task_event(&self, event: MetricsEventKind) {
        event.apply_metrics().await;
        let mut lock = self.events.lock().await;
        lock.push(EventRecord::with_task(
            self.id(),
            self.name(),
            &self.uri,
            event,
            Utc::now(),
        ));
    }

    pub async fn push_service_event(
        &self,
        uri: &Uri,
        service_type: &ServiceType,
        event: MetricsEventKind,
    ) {
        event.apply_metrics().await;
        let mut lock = self.events.lock().await;
        lock.push(EventRecord::with_service(
            self.id(),
            self.name(),
            service_type,
            uri,
            event,
            Utc::now(),
        ));
    }
}

impl fmt::Debug for TaskContext {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("TaskContext")
            .field("id", &self.inner.id)
            .field("started_at", &self.inner.started_at)
            .field("graph", &self.inner.graph)
            .finish()
    }
}

//impl deser / ser

impl<'toy> Deserialize<'toy> for TaskId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct TaskIdVisitor;

        impl<'a> Visitor<'a> for TaskIdVisitor {
            type Value = TaskId;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "error")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(v)
                    .map(TaskId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(&v)
                    .map(TaskId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(v)
                    .map(TaskId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(&v)
                    .map(TaskId::from)
                    .map_err(|e| serde::de::Error::custom(e))
            }
        }

        deserializer.deserialize_string(TaskIdVisitor)
    }
}

impl Serialize for TaskId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.to_string().serialize(serializer)
    }
}

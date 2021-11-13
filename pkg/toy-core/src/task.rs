//! A task is an workflow created based on graph.
//!

use crate::graph::Graph;
use crate::Uri;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;
use std::time::SystemTime;
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
    uri: Option<Uri>,
    current_span: Option<tracing::Span>,
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
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl fmt::Debug for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl Default for TaskId {
    fn default() -> Self {
        TaskId::from(Uuid::nil())
    }
}

macro_rules! task_span {
    ($name: ident, $level: ident) => {
        pub fn $name(&self) -> tracing::span::Span {
            match self.uri() {
              Some(uri) => {
                  tracing::span!(tracing::Level::$level, "Task", task=%self.id(), graph=%self.name(), uri=%uri)
               }
               None => {
                  tracing::span!(tracing::Level::$level, "Task", task=%self.id(), graph=%self.name())
               }
            }
         }
    };
}

impl TaskContext {
    pub fn new(id: TaskId, graph: Graph) -> Self {
        Self {
            inner: Arc::new(Inner {
                id,
                started_at: SystemTime::now(),
                graph,
            }),
            uri: None,
            current_span: None,
        }
    }

    pub fn with_uri(self, uri: &Uri) -> Self {
        Self {
            inner: self.inner,
            uri: Some(uri.clone()),
            current_span: self.current_span,
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

    pub fn uri(&self) -> Option<Uri> {
        self.uri.as_ref().map(|x| x.clone())
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
}

impl fmt::Debug for TaskContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

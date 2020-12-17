use crate::graph::Graph;
use crate::node_channel::SignalOutgoings;
use crate::Uri;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use toy_pack::deser::{Deserializable, Deserializer, Visitor};
use toy_pack::ser::{Serializable, Serializer};
use uuid::Uuid;

/// Task Identifier
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskId {
    id: Uuid,
}

/// You do **not** have to wrap the `TaskContext` it in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
#[derive(Clone)]
pub struct TaskContext {
    inner: Arc<Inner>,
    uri: Option<Uri>,
}

struct Inner {
    id: TaskId,
    started_at: Duration,
    graph: Graph,
}

#[derive(Debug)]
pub struct RunningTask {
    id: TaskId,
    started_at: Duration,
    graph: Graph,

    /// use running task.
    tx_signal: SignalOutgoings,
}

impl TaskId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from(uuid: Uuid) -> Self {
        Self { id: uuid }
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

impl TaskContext {
    pub fn new(graph: Graph) -> Self {
        Self {
            inner: Arc::new(Inner {
                id: TaskId::new(),
                started_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards"),
                graph,
            }),
            uri: None,
        }
    }

    pub fn with_uri(self, uri: &Uri) -> Self {
        Self {
            inner: self.inner,
            uri: Some(uri.clone()),
        }
    }

    pub fn id(&self) -> TaskId {
        self.inner.id
    }

    pub fn started_at(&self) -> Duration {
        self.inner.started_at
    }

    pub fn graph(&self) -> &Graph {
        &self.inner.graph
    }

    pub fn uri(&self) -> Option<Uri> {
        self.uri.as_ref().map(|x| x.clone())
    }

    pub fn trace_span(&self) -> tracing::span::Span {
        match self.uri() {
            Some(uri) => tracing::span!(tracing::Level::TRACE, "Task", task=?self.id(), uri=?uri),
            None => tracing::span!(tracing::Level::TRACE, "Task", task=?self.id()),
        }
    }

    pub fn debug_span(&self) -> tracing::span::Span {
        match self.uri() {
            Some(uri) => tracing::span!(tracing::Level::DEBUG, "Task", task=?self.id(), uri=?uri),
            None => tracing::span!(tracing::Level::DEBUG, "Task", task=?self.id()),
        }
    }

    pub fn info_span(&self) -> tracing::span::Span {
        match self.uri() {
            Some(uri) => tracing::span!(tracing::Level::INFO, "Task", task=?self.id(), uri=?uri),
            None => tracing::span!(tracing::Level::INFO, "Task", task=?self.id()),
        }
    }
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

impl RunningTask {
    pub fn new(ctx: &TaskContext, tx_signal: SignalOutgoings) -> Self {
        Self {
            id: ctx.id(),
            started_at: ctx.started_at(),
            graph: ctx.graph().clone(),
            tx_signal,
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn started_at(&self) -> Duration {
        self.started_at
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn tx_signal(&mut self) -> &mut SignalOutgoings {
        &mut self.tx_signal
    }
}

//impl deser / ser

impl<'toy> Deserializable<'toy> for TaskId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        struct TaskIdVisitor;

        impl<'a> Visitor<'a> for TaskIdVisitor {
            type Value = TaskId;

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: toy_pack::deser::Error,
            {
                Uuid::parse_str(v)
                    .map(TaskId::from)
                    .map_err(|_| toy_pack::deser::Error::invalid_value("[borrowed bytes]", "bytes"))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: toy_pack::deser::Error,
            {
                Uuid::parse_str(&v)
                    .map(TaskId::from)
                    .map_err(|_| toy_pack::deser::Error::invalid_value("[borrowed bytes]", "bytes"))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: toy_pack::deser::Error,
            {
                Uuid::from_slice(v)
                    .map(TaskId::from)
                    .map_err(|_| toy_pack::deser::Error::invalid_value("[borrowed bytes]", "bytes"))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: toy_pack::deser::Error,
            {
                Uuid::from_slice(&v)
                    .map(TaskId::from)
                    .map_err(|_| toy_pack::deser::Error::invalid_value("[borrowed bytes]", "bytes"))
            }
        }

        deserializer.deserialize_string(TaskIdVisitor)
    }
}

impl Serializable for TaskId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.to_string().serialize(serializer)
    }
}

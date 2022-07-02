//! Traits for Basic Key Value Store.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use async_trait::async_trait;
use futures_util::stream::BoxStream;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait Composit store operations.
#[async_trait]
pub trait KvStoreOps<C>:
    Clone
    + Send
    + Sync
    + Find<Con = C>
    + List<Con = C>
    + Put<Con = C>
    + Delete<Con = C>
    + Watch<Con = C>
    + Update<Con = C>
where
    C: StoreConnection,
{
}

/// This trait represents the concept of a Kv Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait KvStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: KvStoreOps<Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

#[derive(Clone, Debug)]
pub struct FindOption {}

impl FindOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct ListOption {
    // selection: field::Selection,
}

impl ListOption {
    pub fn new() -> Self {
        Self {
            // selection: Selection::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PutOperation {
    Fill,
    CreateOnly,
    UpdatedOnly,
}

impl Default for PutOperation {
    fn default() -> Self {
        PutOperation::Fill
    }
}

#[derive(Clone, Debug)]
pub struct PutOption {
    version: u64,
    ops: PutOperation,
}

impl PutOption {
    pub fn new() -> Self {
        Self {
            version: 0,
            ops: PutOperation::Fill,
        }
    }

    pub fn with_create_only(self) -> Self {
        PutOption {
            ops: PutOperation::CreateOnly,
            ..self
        }
    }

    pub fn with_update_only(self) -> Self {
        PutOption {
            ops: PutOperation::UpdatedOnly,
            ..self
        }
    }

    pub fn with_fill(self) -> Self {
        PutOption {
            ops: PutOperation::Fill,
            ..self
        }
    }

    pub fn with_version(self, version: u64) -> Self {
        PutOption { version, ..self }
    }

    pub fn operation(&self) -> PutOperation {
        self.ops
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}

#[derive(Clone, Debug)]
pub struct DeleteOption {}

impl DeleteOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct WatchOption {}

impl WatchOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PutResult {
    Create,
    Update,
}

#[derive(Clone, Copy, Debug)]
pub enum DeleteResult {
    Deleted,
    NotFound,
}

#[derive(Clone, Copy, Debug)]
pub enum UpdateResult {
    Update,
    None,
    NotFound,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KvMeta {
    version: u64,
}

impl KvMeta {
    pub fn new(version: u64) -> Self {
        Self { version }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KvResponse<T> {
    meta: KvMeta,
    value: T,
}

impl<T> KvResponse<T> {
    pub fn new(value: T, meta: KvMeta) -> Self {
        KvResponse { value, meta }
    }

    pub fn with_version(value: T, version: u64) -> Self {
        KvResponse {
            value,
            meta: KvMeta { version },
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn version(&self) -> u64 {
        self.meta.version
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum KvWatchEventType {
    PUT,
    DELETE,
}

#[derive(Clone, Debug)]
pub struct KvWatchEventValue<T> {
    event: KvWatchEventType,
    value: KvResponse<T>,
}

impl<T> KvWatchEventValue<T> {
    pub fn event(&self) -> KvWatchEventType {
        self.event
    }

    pub fn version(&self) -> u64 {
        self.value.version()
    }

    pub fn into_value(self) -> T {
        self.value.into_value()
    }
}

#[derive(Clone, Debug)]
pub struct KvWatchResponse<T> {
    values: Vec<KvWatchEventValue<T>>,
}

impl<T> KvWatchResponse<T> {
    pub fn new(values: impl Iterator<Item = (KvResponse<T>, KvWatchEventType)>) -> Self {
        KvWatchResponse {
            values: values
                .map(|(v, e)| KvWatchEventValue::<T> { event: e, value: v })
                .collect(),
        }
    }

    pub fn into_values(self) -> Vec<KvWatchEventValue<T>> {
        self.values
    }
}

/// Find one entity by specified key.
#[async_trait]
pub trait Find {
    type Con: StoreConnection;

    /// Find one entity by specified key.
    async fn find<V>(
        &self,
        con: Self::Con,
        key: String,
        opt: FindOption,
    ) -> Result<Option<KvResponse<V>>, StoreError>
    where
        V: DeserializeOwned;
}

/// List all or part entities by specified prefix of key.
#[async_trait]
pub trait List {
    type Con: StoreConnection;

    /// List all or part entities by specified prefix of key.
    async fn list<V>(
        &self,
        con: Self::Con,
        prefix: String,
        opt: ListOption,
    ) -> Result<Vec<KvResponse<V>>, StoreError>
    where
        V: DeserializeOwned;
}

/// Put one entity by specified key.
#[async_trait]
pub trait Put {
    type Con: StoreConnection;

    /// Put one entity by specified key.
    async fn put<V>(
        &self,
        con: Self::Con,
        key: String,
        v: V,
        opt: PutOption,
    ) -> Result<PutResult, StoreError>
    where
        V: Serialize + Send;
}

/// Delete one entity by specified key.
#[async_trait]
pub trait Delete {
    type Con: StoreConnection;

    /// Delete one entity by specified key.
    async fn delete(
        &self,
        con: Self::Con,
        key: String,
        opt: DeleteOption,
    ) -> Result<DeleteResult, StoreError>;
}

/// Watch entity by specified key.
#[async_trait]
pub trait Watch {
    type Con: StoreConnection;

    /// Watch entity by specified key.
    async fn watch<V>(
        &self,
        con: Self::Con,
        prefix: String,
        opt: WatchOption,
    ) -> Result<BoxStream<Result<KvWatchResponse<V>, StoreError>>, StoreError>
    where
        V: DeserializeOwned;
}

/// Update entity by specified key.
#[async_trait]
pub trait Update {
    type Con: StoreConnection;

    /// Update entity by specified key.
    async fn update<V, F>(
        &self,
        con: Self::Con,
        key: String,
        f: F,
    ) -> Result<UpdateResult, StoreError>
    where
        V: DeserializeOwned + Serialize + Send,
        F: FnOnce(V) -> Option<V> + Send;
}

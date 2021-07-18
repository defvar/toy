//! Traits for Basic Key Value Store.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use async_trait::async_trait;
use std::fmt::Debug;
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;

/// Trait Composit store operations.
#[async_trait]
pub trait KvStoreOps<C>:
    Clone + Send + Sync + Find<Con = C> + List<Con = C> + Put<Con = C> + Delete<Con = C>
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
pub struct ListOption {}

impl ListOption {
    pub fn new() -> Self {
        Self {}
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
    ) -> Result<Option<V>, StoreError>
    where
        V: DeserializableOwned;
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
    ) -> Result<Vec<V>, StoreError>
    where
        V: DeserializableOwned;
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
        V: Serializable + Send;
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

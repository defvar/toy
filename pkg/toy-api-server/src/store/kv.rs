//! Traits for Basic Key Value Store.

use crate::store::StoreConnection;
use async_trait::async_trait;
use std::fmt::Debug;
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;

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

#[derive(Clone, Debug)]
pub struct PutOption {}

impl PutOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct DeleteOption {}

impl DeleteOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug)]
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
    type Err: Debug + Send;

    /// Find one entity by specified key.
    async fn find<V>(
        &self,
        con: Self::Con,
        key: String,
        opt: FindOption,
    ) -> Result<Option<V>, Self::Err>
    where
        V: DeserializableOwned;
}

/// List all or part entities by specified prefix of key.
#[async_trait]
pub trait List {
    type Con: StoreConnection;
    type Err: Debug + Send;

    /// List all or part entities by specified prefix of key.
    async fn list<V>(
        &self,
        con: Self::Con,
        prefix: String,
        opt: ListOption,
    ) -> Result<Vec<V>, Self::Err>
    where
        V: DeserializableOwned;
}

/// Put one entity by specified key.
#[async_trait]
pub trait Put {
    type Con: StoreConnection;
    type Err: Debug + Send;

    /// Put one entity by specified key.
    async fn put<V>(
        &self,
        con: Self::Con,
        key: String,
        v: V,
        opt: PutOption,
    ) -> Result<PutResult, Self::Err>
    where
        V: Serializable + Send;
}

/// Delete one entity by specified key.
#[async_trait]
pub trait Delete {
    type Con: StoreConnection;
    type Err: Debug + Send;

    /// Delete one entity by specified key.
    async fn delete(
        &self,
        con: Self::Con,
        key: String,
        opt: DeleteOption,
    ) -> Result<DeleteResult, Self::Err>;
}

use crate::store::StoreConnection;
use std::fmt::Debug;
use std::future::Future;
use toy_core::data::Value;

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

#[derive(Clone, Copy, Debug)]
pub enum PutResult {
    Create,
    Update,
}

#[derive(Clone, Debug)]
pub struct DeleteOption {}

impl DeleteOption {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DeleteResult {
    Deleted,
    NotFound,
}

/// Find one entity by specified key.
pub trait Find {
    type Con: StoreConnection;
    type T: Future<Output = Result<Option<Value>, Self::Err>> + Send;
    type Err: Debug + Send;

    fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T;
}

/// List all or part entities by specified prefix of key.
pub trait List {
    type Con: StoreConnection;
    type T: Future<Output = Result<Vec<Value>, Self::Err>> + Send;
    type Err: Debug + Send;

    fn list(&self, con: Self::Con, prefix: String, opt: ListOption) -> Self::T;
}

/// Put one entity by specified key.
pub trait Put {
    type Con: StoreConnection;
    type T: Future<Output = Result<PutResult, Self::Err>> + Send;
    type Err: Debug + Send;

    fn put(&self, con: Self::Con, key: String, v: Value, opt: PutOption) -> Self::T;
}

/// Delete one entity by specified key.
pub trait Delete {
    type Con: StoreConnection;
    type T: Future<Output = Result<DeleteResult, Self::Err>> + Send;
    type Err: Debug + Send;

    fn delete(&self, con: Self::Con, key: String, opt: DeleteOption) -> Self::T;
}

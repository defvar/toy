//! Trait for metrics operations.

use crate::store::error::StoreError;
use crate::store::StoreConnection;
use std::fmt;
use toy_api::metrics::Metrics;

/// This trait represents the concept of a metrics Store.
///
///  - Create or get establish connection.
///  - Get composit operation trait.
pub trait MetricsStore<T>: Clone + Send + Sync {
    type Con: StoreConnection;
    type Ops: MetricsStoreOps<Con = Self::Con>;

    fn con(&self) -> Option<Self::Con>;

    fn ops(&self) -> Self::Ops;

    fn establish(&mut self, client: T) -> Result<(), StoreError>;
}

/// Trait metrics store operations.
#[async_trait::async_trait]
pub trait MetricsStoreOps: Send + Sync {
    type Con: StoreConnection;
    type Err: fmt::Debug + Send;

    async fn create(
        &self,
        con: Self::Con,
        metrics: Metrics,
        opt: CreateOption,
    ) -> Result<(), Self::Err>;
}

//////////////////////////
// option
//////////////////////////

#[derive(Clone, Debug)]
pub struct CreateOption {}

impl CreateOption {
    pub fn new() -> Self {
        Self {}
    }
}

//////////////////////////
// noop store
//////////////////////////

#[derive(Clone)]
pub struct NoopMetricsStore;

impl StoreConnection for NoopMetricsStore {}

#[async_trait::async_trait]
impl MetricsStoreOps for NoopMetricsStore {
    type Con = Self;
    type Err = StoreError;

    async fn create(
        &self,
        _con: Self::Con,
        _metrics: Metrics,
        _opt: CreateOption,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
}

impl<T> MetricsStore<T> for NoopMetricsStore {
    type Con = Self;
    type Ops = Self;

    fn con(&self) -> Option<Self::Con> {
        Some(Self)
    }

    fn ops(&self) -> Self::Ops {
        NoopMetricsStore
    }

    fn establish(&mut self, _client: T) -> Result<(), StoreError> {
        Ok(())
    }
}

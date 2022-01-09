use crate::context::Context;
use crate::store::kv::KvStore;
use crate::ApiError;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use toy_h::HttpClient;

#[async_trait]
pub trait Validator<H, Store, V>
where
    H: HttpClient,
    Store: KvStore<H>,
    V: DeserializeOwned + Serialize + Send,
{
    async fn validate(&self, ctx: &Context, store: &Store, v: V) -> Result<V, ApiError>;
}

pub struct OkValidator<V> {
    t: PhantomData<V>,
}

impl<V> OkValidator<V> {
    pub fn new() -> Self {
        Self { t: PhantomData }
    }
}

#[async_trait]
impl<H, Store, V> Validator<H, Store, V> for OkValidator<V>
where
    H: HttpClient,
    Store: KvStore<H>,
    V: DeserializeOwned + Serialize + Send + Sync,
{
    async fn validate(&self, _ctx: &Context, _store: &Store, v: V) -> Result<V, ApiError> {
        Ok(v)
    }
}

use crate::context::Context;
use crate::store::kv::KvStore;
use crate::ApiError;
use std::marker::PhantomData;
use toy_h::HttpClient;

pub trait Validator<H, Store, V>
where
    Store: KvStore<H>,
    H: HttpClient,
{
    fn validate(ctx: &Context, store: &Store, v: V) -> Result<V, ApiError>;
}

pub struct OkValidator<V> {
    t: PhantomData<V>,
}

impl<H, Store, V> Validator<H, Store, V> for OkValidator<V>
where
    Store: KvStore<H>,
    H: HttpClient,
{
    fn validate(_ctx: &Context, _store: &Store, v: V) -> Result<V, ApiError> {
        Ok(v)
    }
}

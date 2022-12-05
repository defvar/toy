use crate::async_trait::async_trait;
use crate::authentication::{Auth, AuthUser};
use crate::store::kv::KvStore;
use crate::store::task_event::TaskEventStore;
use crate::ApiError;
use toy_api_http_common::axum::{extract::FromRequestParts, http::request::Parts};
use toy_h::HttpClient;

pub mod dispatcher;
pub mod rbac;
pub mod server;
pub mod supervisor_cleaner;

pub trait ServerState: Clone + Send + Sync {
    type Client: HttpClient;
    type Auth: Auth<Self::Client> + 'static;
    type KvStore: KvStore<Self::Client> + 'static;
    type TaskEventStore: TaskEventStore<Self::Client> + 'static;

    fn client(&self) -> &Self::Client;

    fn auth(&self) -> &Self::Auth;

    fn kv_store(&self) -> &Self::KvStore;

    fn kv_store_mut(&mut self) -> &mut Self::KvStore;

    fn task_event_store(&self) -> &Self::TaskEventStore;

    fn task_event_store_mut(&mut self) -> &mut Self::TaskEventStore;
}

#[derive(Debug, Clone)]
pub struct WrappedState<S>
where
    S: ServerState,
{
    raw: S,
}

impl<S> WrappedState<S>
where
    S: ServerState,
{
    pub fn new(state: S) -> Self {
        Self { raw: state }
    }

    pub fn raw(&self) -> &S {
        &self.raw
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    user: AuthUser,
    resource: String,
    verb: String,
}

impl Context {
    pub fn new<T: Into<String>>(user: AuthUser, resource: T, verb: T) -> Self {
        Context {
            user,
            resource: resource.into(),
            verb: verb.into(),
        }
    }

    pub fn user(&self) -> &AuthUser {
        &self.user
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn verb(&self) -> &str {
        &self.verb
    }
}

#[async_trait]
impl<S> FromRequestParts<WrappedState<S>> for Context
where
    S: ServerState,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &WrappedState<S>,
    ) -> Result<Self, Self::Rejection> {
        let context_or_error = crate::authentication::authenticate(parts, &state.raw).await;
        match context_or_error {
            Ok(ctx) => {
                let roles = rbac::rules(ctx.user().name());
                let r = crate::authorization::authorize(&ctx, roles);
                if let Err(e) = r {
                    Err(e)
                } else {
                    Ok(ctx)
                }
            }
            Err(e) => Err(e),
        }
    }
}

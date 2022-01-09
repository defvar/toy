use crate::common::validator::Validator;
use crate::context::Context;
use crate::store::kv::KvStore;
use crate::ApiError;
use async_trait::async_trait;
use toy_api::role::Role;
use toy_h::HttpClient;

pub struct RoleValidator;

#[async_trait]
impl<H, Store> Validator<H, Store, Role> for RoleValidator
where
    H: HttpClient,
    Store: KvStore<H>,
{
    async fn validate(&self, _ctx: &Context, _store: &Store, v: Role) -> Result<Role, ApiError> {
        if v.name() == "system" {
            Err(ApiError::error(format!(
                "\"{:?}\" invalid role name.",
                v.name()
            )))
        } else {
            Ok(v)
        }
    }
}

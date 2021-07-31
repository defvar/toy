use crate::common::validator::Validator;
use crate::context::Context;
use crate::store::kv::KvStore;
use crate::ApiError;
use toy_api::role::Role;
use toy_h::HttpClient;

pub struct RoleValidator;

impl<H, Store> Validator<H, Store, Role> for RoleValidator
where
    Store: KvStore<H>,
    H: HttpClient,
{
    fn validate(_ctx: &Context, _store: &Store, v: Role) -> Result<Role, ApiError> {
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

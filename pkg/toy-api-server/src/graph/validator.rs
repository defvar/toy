use crate::common::validator::Validator;
use crate::context::Context;
use crate::store::kv::{Find, FindOption, KvStore};
use crate::ApiError;
use async_trait::async_trait;
use toy_api::graph::Graph;
use toy_h::HttpClient;

pub struct GraphPutValidator;

#[async_trait]
impl<H, Store> Validator<H, Store, Graph> for GraphPutValidator
where
    H: HttpClient,
    Store: KvStore<H>,
{
    async fn validate(&self, _ctx: &Context, store: &Store, v: Graph) -> Result<Graph, ApiError> {
        if v.name().len() == 0 {
            return Err(ApiError::validation_failed("\"name\" is required."));
        }
        if v.services().len() == 0 {
            return Err(ApiError::validation_failed("\"services\" is required."));
        }

        // graph name duplicate check
        let dup = match store
            .ops()
            .find::<Graph>(store.con().unwrap(), v.name().to_owned(), FindOption::new())
            .await
        {
            Ok(v) => match v {
                Some(_) => true,
                None => false,
            },
            Err(e) => {
                tracing::error!("error:{:?}", e);
                return Err(ApiError::store_operation_failed(e));
            }
        };
        if dup {
            return Err(ApiError::validation_failed(format!(
                "\"name\":\"{}\" already exists.",
                v.name()
            )));
        }

        // TODO: service exists check

        // TODO: service schema check

        // deserialize check
        let value = toy_core::data::pack(v.clone())?;
        let _ = toy_core::graph::Graph::from(value).map_err(|e| ApiError::validation_failed(e))?;
        Ok(v)
    }
}

use crate::common::validator::Validator;
use crate::context::Context;
use crate::store::error::StoreError;
use crate::store::kv;
use crate::store::kv::{Delete, DeleteResult, Find, KvResponse, KvStore, List, Put};
use crate::{common, ApiError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use toy_api::common::{
    CommonPutResponse, DeleteOption, FindOption, KVObject, ListObject, ListOptionLike, PutOption,
    SelectionCandidate,
};
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::{codec, reply};
use toy_h::{Bytes, HttpClient, StatusCode};

pub async fn find2<H, V, R, F>(
    ctx: Context,
    store: &impl KvStore<H>,
    key: String,
    api_opt: FindOption,
    store_opt: kv::FindOption,
    f: F,
) -> Result<impl IntoResponse, ApiError>
where
    H: HttpClient,
    F: FnOnce(V) -> R,
    V: DeserializeOwned,
    R: Serialize,
{
    tracing::debug!("handle: ctx:{:?}, opt:{:?}", ctx, api_opt);

    match store
        .ops()
        .find::<V>(store.con().unwrap(), key, store_opt)
        .await
    {
        Ok(v) => match v {
            Some(v) => {
                let format = api_opt.format();
                let indent = api_opt.indent();
                let fields = api_opt.fields();
                let r = f(v.into_value());
                if fields.is_empty() {
                    Ok(reply::into_response(&r, format, indent))
                } else {
                    let value = toy_core::data::pack(r)?;
                    let applied_value = fields
                        .apply(&value)
                        .map_err(|e| ApiError::invalid_field(e))?;
                    Ok(reply::into_response(&applied_value, format, indent))
                }
            }
            None => Err(ApiError::error("not found")),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

pub async fn list2<H, V, R, Opt, StoreOptF, F>(
    ctx: Context,
    store: &impl KvStore<H>,
    prefix: &str,
    api_opt: Opt,
    store_opt_f: StoreOptF,
    f: F,
) -> Result<impl IntoResponse, ApiError>
where
    H: HttpClient,
    V: Serialize + DeserializeOwned + SelectionCandidate,
    R: Serialize + ListObject<V>,
    Opt: ListOptionLike + Debug,
    StoreOptF: FnOnce(&Opt) -> kv::ListOption,
    F: FnOnce(Vec<V>) -> R,
{
    tracing::debug!("handle: ctx:{:?}, opt:{:?}", ctx, api_opt);

    match store
        .ops()
        .list::<V>(
            store.con().unwrap(),
            prefix.to_owned(),
            store_opt_f(&api_opt),
        )
        .await
    {
        Ok(mut vec) => {
            let selector = api_opt.common().selection();
            let fields = api_opt.common().fields();
            let format = api_opt.common().format();
            let indent = api_opt.common().indent();

            if !selector.preds().is_empty() {
                // check fields
                selector
                    .validation_fields::<V>()
                    .map_err(|e| ApiError::invalid_selectors(e))?;

                // filter
                let filterd: Result<Vec<KvResponse<V>>, String> = vec
                    .into_iter()
                    .filter_map(|item| match selector.is_match(item.value()) {
                        Ok(true) => Some(Ok(item)),
                        Ok(false) => None,
                        Err(name) => Some(Err(name)),
                    })
                    .collect();

                match filterd {
                    Ok(v) => vec = v,
                    Err(name) => return Err(ApiError::invalid_selector(name)),
                }
            }

            let r = f(vec.into_iter().map(|x| x.into_value()).collect());
            if fields.is_empty() {
                Ok(reply::into_response(&r, format, indent))
            } else {
                let result_list = r.items().iter().try_fold(
                    Vec::with_capacity(r.count() as usize),
                    |mut acc, item| {
                        let value = toy_core::data::pack(item)?;
                        match fields.apply(&value) {
                            Ok(applied_value) => {
                                acc.push(applied_value);
                                Ok(acc)
                            }
                            Err(f) => Err(ApiError::invalid_field(f)),
                        }
                    },
                )?;
                Ok(reply::into_response(&result_list, format, indent))
            }
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

pub async fn put2<H, Store, Req, T>(
    ctx: Context,
    store: &Store,
    prefix: &'static str,
    key: String,
    opt: PutOption,
    store_opt: kv::PutOption,
    request: Bytes,
    validator: T,
) -> Result<impl IntoResponse, ApiError>
where
    H: HttpClient,
    Req: DeserializeOwned + Serialize + KVObject + Send,
    Store: KvStore<H>,
    T: Validator<H, Store, Req>,
{
    tracing::debug!("handle: {:?}, opt: {:?}", ctx, opt);

    let format = opt.format();
    let v = codec::decode::<_, Req>(request, format)?;
    let key_of_data = KVObject::key(&v).to_owned();

    if key_of_data != key {
        return Err(ApiError::difference_key(&key, &key_of_data));
    }

    let v = validator.validate(&ctx, &store, v).await?;
    match store
        .ops()
        .put(
            store.con().unwrap(),
            common::constants::generate_key(prefix, &key),
            v,
            store_opt,
        )
        .await
    {
        Ok(_) => {
            let r = CommonPutResponse::with_code(StatusCode::CREATED.as_u16());
            let r = reply::into_response(&r, format, opt.indent());
            Ok((StatusCode::CREATED, r))
        }
        Err(e) => match e {
            StoreError::AllreadyExists { .. } => Err(ApiError::allready_exists(&key)),
            _ => {
                tracing::error!("error:{:?}", e);
                Err(ApiError::store_operation_failed("internal error..."))
            }
        },
    }
}

pub async fn delete2<H>(
    ctx: Context,
    store: &impl KvStore<H>,
    key: String,
    _api_opt: DeleteOption,
    store_opt: kv::DeleteOption,
) -> Result<impl IntoResponse, ApiError>
where
    H: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);
    match store
        .ops()
        .delete(store.con().unwrap(), key, store_opt)
        .await
    {
        Ok(r) => match r {
            DeleteResult::Deleted => Ok(StatusCode::NO_CONTENT),
            DeleteResult::NotFound => Ok(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

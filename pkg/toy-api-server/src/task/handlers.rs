use crate::common::constants;
use crate::context::Context;
use crate::store::kv::{KvStore, Put, PutOption, PutResult, Update, UpdateResult};
use crate::store::task_event::{
    CreateOption, ListEventOption, ListTaskOption, TaskEventStore, TaskEventStoreOps,
};
use crate::ApiError;
use chrono::{DateTime, Duration, Utc};
use toy_api::common::{self as api_common, CommonPostResponse, ListOptionLike};
use toy_api::graph::Graph;
use toy_api::selection::selector::Predicate;
use toy_api::selection::Operator;
use toy_api::task::{
    FinishResponse, PendingResult, PendingTask, TaskEvent, TaskEventListOption, TaskListOption,
};
use toy_api_http_common::axum::http::StatusCode;
use toy_api_http_common::axum::response::IntoResponse;
use toy_api_http_common::bytes::Bytes;
use toy_api_http_common::{codec, reply};
use toy_core::task::TaskId;
use toy_h::HttpClient;

pub async fn post<T>(
    ctx: Context,
    opt: api_common::PostOption,
    request: Bytes,
    store: &impl KvStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let format = opt.format();
    let v = codec::decode::<_, Graph>(request, format)?;

    /*
     validate...
    */

    let id = TaskId::new();
    let pending = PendingTask::new(id, v);
    let key = constants::pending_key(id);
    match store
        .ops()
        .put(
            store.con().unwrap(),
            key,
            pending,
            PutOption::new().with_create_only(),
        )
        .await
    {
        Ok(PutResult::Create) => Ok(reply::into_response(
            &(PendingResult::from_id(id)),
            format,
            None,
        )),
        Ok(PutResult::Update(_)) => unreachable!(),
        Err(e) => Err(ApiError::store_operation_failed(e)),
    }
}

pub async fn finish<T>(
    ctx: Context,
    store: &impl KvStore<T>,
    key: String,
    api_opt: api_common::PostOption,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let id = TaskId::parse_str(&key);
    if id.is_err() {
        return Err(ApiError::task_id_invalid_format(key));
    }
    let id = id.unwrap();

    let format = api_opt.format();
    let key = constants::generate_key(constants::PENDINGS_KEY_PREFIX, key);
    let now = Utc::now();
    let f = |v: PendingTask| Some(v.finished(now));
    match store.ops().update(store.con().unwrap(), key, f).await {
        Ok(UpdateResult::Update(_)) => {
            Ok(reply::into_response(&FinishResponse::ok(id), format, None))
        }
        Ok(UpdateResult::NotFound) => Ok(reply::into_response(
            &FinishResponse::not_found(id),
            format,
            None,
        )),
        Ok(UpdateResult::None) => unreachable!(),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::error(e))
        }
    }
}

pub async fn list_task<T>(
    ctx: Context,
    opt: TaskListOption,
    log_store: &impl TaskEventStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);
    opt.common()
        .selection()
        .validation_fields_by_names(&["name", "start", "stop"])
        .map_err(|e| ApiError::invalid_selectors(e))?;

    let name = opt.common().selection().get("name").map(|x| x.clone());
    let start = opt.common().selection().get("start").map(|x| x.clone());
    let stop = opt.common().selection().get("stop").map(|x| x.clone());

    let (start, stop) = to_time_predicate(start, stop)?;

    match log_store
        .ops()
        .list_task(
            log_store.con().unwrap(),
            ListTaskOption::with(name, start, stop, opt.common().limit()),
        )
        .await
    {
        Ok(v) => Ok(reply::into_list_item_response_with_fields(
            &v,
            opt.common().format(),
            opt.common().indent(),
            opt.common().fields(),
        )),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

pub async fn post_task_event<T>(
    ctx: Context,
    opt: api_common::PostOption,
    request: Bytes,
    event_store: &impl TaskEventStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}", ctx);

    let format = opt.format();
    let v = codec::decode::<_, Vec<TaskEvent>>(request, format)?;

    match event_store
        .ops()
        .create(event_store.con().unwrap(), v, CreateOption::new())
        .await
    {
        Ok(()) => {
            let r = CommonPostResponse::with_code(StatusCode::CREATED.as_u16());
            let r = reply::into_response(&r, format, opt.indent());
            Ok((StatusCode::CREATED, r))
        }
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

pub async fn list_task_event<T>(
    ctx: Context,
    opt: TaskEventListOption,
    event_store: &impl TaskEventStore<T>,
) -> Result<impl IntoResponse, ApiError>
where
    T: HttpClient,
{
    tracing::debug!("handle: {:?}, opt: {:?}", ctx, opt);

    opt.common()
        .selection()
        .validation_fields_by_names(&["name", "start", "stop"])
        .map_err(|e| ApiError::invalid_selectors(e))?;

    let name = opt.common().selection().get("name").map(|x| x.clone());
    let start = opt.common().selection().get("start").map(|x| x.clone());
    let stop = opt.common().selection().get("stop").map(|x| x.clone());

    let (start, stop) = to_time_predicate(start, stop)?;

    match event_store
        .ops()
        .list_event(
            event_store.con().unwrap(),
            ListEventOption::with(name, start, stop, opt.common().limit()),
        )
        .await
    {
        Ok(v) => Ok(reply::into_list_item_response_with_fields(
            &v,
            opt.common().format(),
            opt.common().indent(),
            opt.common().fields(),
        )),
        Err(e) => {
            tracing::error!("error:{:?}", e);
            Err(ApiError::store_operation_failed(e))
        }
    }
}

fn to_time_predicate(
    start: Option<Predicate>,
    stop: Option<Predicate>,
) -> Result<(Option<DateTime<Utc>>, Option<DateTime<Utc>>), ApiError> {
    let now = Utc::now();
    let start = if let Some(start) = start {
        match start.op() {
            Operator::Eq => Ok(start.value().as_timestamp()),
            _ => Err(ApiError::error("selector: [start] must be eq.")),
        }
    } else {
        Ok(Some(now - Duration::hours(1)))
    }?;

    let stop = if let Some(stop) = stop {
        match stop.op() {
            Operator::Eq => Ok(stop.value().as_timestamp()),
            _ => Err(ApiError::error("selector: [stop] must be eq.")),
        }
    } else {
        Ok(Some(now))
    }?;

    Ok((start, stop))
}

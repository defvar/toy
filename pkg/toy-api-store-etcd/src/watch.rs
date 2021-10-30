use crate::kv::{encode, get_range_end, Kv, ResponseHeader, Versioning};
use serde::{Deserialize, Serialize};
use toy_api_server::store::error::StoreError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    NOPUT,
    NODELETE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    PUT,
    DELETE,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    #[serde(default)]
    tp: EventType,
    kv: Kv,
    prev_kv: Option<Kv>,
}

#[derive(Debug, Serialize)]
pub struct WatchCreateRequest {
    create_request: WatchCreateRequestInner,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct WatchCreateRequestInner {
    key: String,
    range_end: Option<String>,
    start_revision: Option<u64>,
    progress_notify: Option<bool>,
    filters: Option<Vec<FilterType>>,
    prev_kv: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct WatchCancelRequest {
    watch_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct WatchResponse {
    result: Option<WatchResponseInner>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WatchResponseInner {
    header: ResponseHeader,
    watch_id: Option<u64>,
    #[serde(default)]
    created: bool,
    #[serde(default)]
    canceled: bool,
    #[serde(default)]
    compact_revision: u64,
    #[serde(default)]
    events: Vec<Event>,
}

impl Default for EventType {
    fn default() -> Self {
        EventType::PUT
    }
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::NOPUT
    }
}

impl WatchCreateRequest {
    pub fn single(key: &str) -> WatchCreateRequest {
        let encoded_key = encode(key);
        WatchCreateRequest {
            create_request: WatchCreateRequestInner {
                key: encoded_key,
                range_end: None,
                start_revision: None,
                progress_notify: None,
                filters: None,
                prev_kv: None,
            },
        }
    }

    pub fn range_from(key: &str) -> WatchCreateRequest {
        let encoded_key = encode(key);
        let range_end = {
            std::str::from_utf8(get_range_end(key).as_slice())
                .map(|x| encode(x.to_string()))
                .ok()
        };
        WatchCreateRequest {
            create_request: WatchCreateRequestInner {
                key: encoded_key,
                range_end,
                start_revision: None,
                progress_notify: None,
                filters: None,
                prev_kv: None,
            },
        }
    }
}

impl WatchResponse {
    pub fn unpack<T, F>(&self, mut f: F) -> Result<Vec<T>, StoreError>
    where
        F: FnMut(Versioning, EventType) -> Option<Result<T, StoreError>>,
    {
        match self.result {
            Some(ref inner) => inner.events.iter().try_fold(Vec::new(), |mut vec, x| {
                let v = x.kv.to_versioning().map_err(|e| StoreError::error(e))?;
                match f(v, x.tp) {
                    Some(v) => {
                        vec.push(v?);
                        Ok(vec)
                    }
                    None => Ok(vec),
                }
            }),
            None => Ok(Vec::new()),
        }
    }
}

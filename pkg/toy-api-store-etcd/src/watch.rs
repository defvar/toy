use crate::error::StoreEtcdError;
use crate::kv::{encode, get_range_end, Kv, ResponseHeader, Versioning};
use toy_pack::deser::DeserializableOwned;
use toy_pack::{Pack, Unpack};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, Unpack)]
pub enum FilterType {
    NOPUT,
    NODELETE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pack, Unpack)]
pub enum EventType {
    PUT,
    DELETE,
}

#[derive(Debug, Unpack)]
pub struct Event {
    #[toy(rename = "type")]
    tp: EventType,
    kv: Kv,
    prev_kv: Kv,
}

#[derive(Debug, Pack)]
pub struct WatchCreateRequest {
    create_request: WatchCreateRequestInner,
}

#[derive(Debug, Pack)]
#[toy(ignore_pack_if_none)]
struct WatchCreateRequestInner {
    key: String,
    range_end: Option<String>,
    start_revision: Option<u64>,
    progress_notify: Option<bool>,
    filters: Option<Vec<FilterType>>,
    prev_kv: Option<bool>,
}

#[derive(Debug, Pack)]
pub struct WatchCancelRequest {
    watch_id: u64,
}

#[derive(Debug, Unpack)]
pub struct WatchResponse {
    result: Option<WatchResponseInner>,
}

#[derive(Debug, Unpack)]
struct WatchResponseInner {
    header: ResponseHeader,
    watch_id: u64,
    created: bool,
    canceled: bool,
    compact_revision: u64,
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
    pub fn unpack<T, F>(&self, mut f: F) -> Result<Vec<T>, StoreEtcdError>
    where
        T: DeserializableOwned,
        F: FnMut(Versioning, EventType) -> Option<Result<T, StoreEtcdError>>,
    {
        match self.result {
            Some(ref inner) => inner.events.iter().try_fold(Vec::new(), |mut vec, x| {
                let v = x.kv.to_versioning()?;
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

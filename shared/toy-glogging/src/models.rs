use std::collections::HashMap;
use toy_pack::{Pack, Unpack};

#[derive(Clone, Copy, Debug, Pack, Unpack)]
pub enum Severity {
    DEFAULT,
    INFO,
    DEBUG,
    ERROR,
}

#[derive(Clone, Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct Entry {
    #[toy(rename = "logName")]
    log_name: String,
    resource: Resource,
    timestamp: Option<String>,
    #[toy(rename = "receiveTimestamp")]
    receive_timestamp: Option<String>,
    severity: Option<Severity>,
    #[toy(rename = "insertId")]
    insert_id: Option<String>,
    labels: Option<HashMap<String, String>>,
    #[toy(rename = "jsonPayload")]
    json_payload: Option<HashMap<String, String>>,
}

pub struct EntryBuilder {
    e: Entry,
}

#[derive(Default, Clone, Debug, Pack, Unpack)]
#[toy(ignore_pack_if_none)]
pub struct Resource {
    #[toy(rename = "type")]
    tp: String,
    labels: Option<HashMap<String, String>>,
}

#[derive(Default, Clone, Debug, Unpack)]
pub struct ErrorResponse {
    error: ErrorInfo,
}

#[derive(Default, Clone, Debug, Unpack)]
pub struct ErrorInfo {
    code: u32,
    message: String,
    status: String,
    details: Vec<ErrorDetail>,
}

#[derive(Default, Clone, Debug, Unpack)]
pub struct ErrorDetail {
    #[toy(rename = "@type")]
    tp: String,
    #[toy(rename = "fieldViolations")]
    field_violations: Vec<FieldViolation>,
}

#[derive(Default, Clone, Debug, Unpack)]
pub struct FieldViolation {
    description: String,
}

///////////////////////////////
// List
///////////////////////////////

#[derive(Clone, Debug, Pack)]
#[toy(ignore_pack_if_none)]
pub struct ListRequest {
    #[toy(rename = "resourceNames")]
    resource_names: Vec<String>,
    filter: Option<String>,
    #[toy(rename = "orderBy")]
    order_by: Option<String>,
    #[toy(rename = "pageSize")]
    page_size: Option<u32>,
    #[toy(rename = "pageToken")]
    page_token: Option<String>,
}

#[derive(Clone, Debug, Unpack)]
pub struct ListResponse {
    entries: Vec<Entry>,
    #[toy(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

///////////////////////////////
// Write
///////////////////////////////

#[derive(Clone, Debug, Pack)]
#[toy(ignore_pack_if_none)]
pub struct WriteRequest {
    #[toy(rename = "logName")]
    log_name: Option<String>,
    resource: Option<Resource>,
    labels: Option<HashMap<String, String>>,
    entries: Vec<Entry>,
    #[toy(rename = "partialSuccess")]
    partial_success: Option<bool>,
    #[toy(rename = "dryRun")]
    dry_run: Option<bool>,
}

#[derive(Clone, Debug, Unpack)]
pub struct WriteResponse;

///////////////////////////////
// Tail
///////////////////////////////
#[derive(Clone, Debug, Pack)]
#[toy(ignore_pack_if_none)]
pub struct TailRequest {
    #[toy(rename = "resourceNames")]
    resource_names: Vec<String>,
    filter: Option<String>,
    #[toy(rename = "bufferWindow")]
    buffer_window: Option<TailDuration>,
}

#[derive(Default, Clone, Debug, Pack)]
pub struct TailDuration {
    seconds: u64,
    nanos: u32,
}

#[derive(Clone, Debug, Unpack)]
pub struct TailResponse {
    entries: Vec<Entry>,
    #[toy(rename = "suppressionInfo")]
    suppression_info: SuppressionInfo,
}

#[derive(Default, Clone, Debug, Unpack)]
pub struct SuppressionInfo {
    reason: SuppressionReason,
    #[toy(rename = "suppressedCount")]
    suppressed_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Unpack)]
pub enum SuppressionReason {
    #[toy(rename = "REASON_UNSPECIFIED")]
    ReasonUnspecified,
    #[toy(rename = "RATE_LIMIT")]
    RateLimit,
    #[toy(rename = "NOT_CONSUMED")]
    NotConsumed,
}

///////////////////////////////
// impl
///////////////////////////////

impl Default for Severity {
    fn default() -> Self {
        Severity::DEFAULT
    }
}

impl Default for SuppressionReason {
    fn default() -> Self {
        SuppressionReason::ReasonUnspecified
    }
}

impl Resource {
    pub fn new<T: Into<String>>(tp: T) -> Resource {
        Resource {
            tp: tp.into(),
            labels: None,
        }
    }
}

impl EntryBuilder {
    pub fn new<T: Into<String>>(log_name: T, resource: Resource) -> EntryBuilder {
        EntryBuilder {
            e: Entry {
                log_name: log_name.into(),
                resource,
                timestamp: None,
                receive_timestamp: None,
                severity: None,
                insert_id: None,
                labels: None,
                json_payload: None,
            },
        }
    }

    pub fn timestamp<T: Into<String>>(mut self, timestamp: T) -> EntryBuilder {
        self.e.timestamp = Some(timestamp.into());
        self
    }

    pub fn severity(mut self, severity: Severity) -> EntryBuilder {
        self.e.severity = Some(severity);
        self
    }

    /// replace json payload
    pub fn json_payload(mut self, map: HashMap<String, String>) -> EntryBuilder {
        self.e.json_payload = Some(map);
        self
    }

    /// push json payload
    pub fn kv<K: Into<String>, V: Into<String>>(mut self, k: K, v: V) -> EntryBuilder {
        if self.e.json_payload.is_none() {
            self.e.json_payload = Some(HashMap::new());
        }
        self.e
            .json_payload
            .as_mut()
            .map(|x| x.insert(k.into(), v.into()));
        self
    }

    /// push json payload
    pub fn kv_opt<K: Into<String>, V: Into<String>>(self, k: K, v: Option<V>) -> EntryBuilder {
        match v {
            Some(v) => self.kv(k, v),
            None => self,
        }
    }

    /// replace labels
    pub fn labels(mut self, map: HashMap<String, String>) -> EntryBuilder {
        self.e.labels = Some(map);
        self
    }

    /// push label
    pub fn label<K: Into<String>, V: Into<String>>(mut self, k: K, v: V) -> EntryBuilder {
        if self.e.labels.is_none() {
            self.e.labels = Some(HashMap::new());
        }
        self.e.labels.as_mut().map(|x| x.insert(k.into(), v.into()));
        self
    }

    /// push label
    pub fn label_opt<K: Into<String>, V: Into<String>>(self, k: K, v: Option<V>) -> EntryBuilder {
        match v {
            Some(v) => self.label(k, v),
            None => self,
        }
    }

    /// build `Entry`
    pub fn build(self) -> Entry {
        self.e
    }
}

impl ErrorResponse {
    pub fn into_error_info(self) -> ErrorInfo {
        self.error
    }
}

impl ListRequest {
    pub fn from_resource_name<T: Into<String>>(r: T) -> ListRequest {
        Self {
            resource_names: vec![r.into()],
            filter: None,
            order_by: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn with_filter<T: Into<String>>(self, filter: T) -> ListRequest {
        Self {
            filter: Some(filter.into()),
            ..self
        }
    }

    pub fn with_order_by<T: Into<String>>(self, order_by: T) -> ListRequest {
        Self {
            order_by: Some(order_by.into()),
            ..self
        }
    }

    pub fn with_page_size(self, page_size: u32) -> ListRequest {
        Self {
            page_size: Some(page_size),
            ..self
        }
    }

    pub fn with_page_token<T: Into<String>>(self, page_token: T) -> ListRequest {
        Self {
            page_token: Some(page_token.into()),
            ..self
        }
    }
}

impl ListResponse {
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_ref().map(|x| x.as_ref())
    }
}

impl WriteRequest {
    pub fn from_entry(entry: Entry) -> WriteRequest {
        WriteRequest::from_entries(vec![entry])
    }

    pub fn from_entries(entries: Vec<Entry>) -> WriteRequest {
        WriteRequest {
            entries,
            log_name: None,
            labels: None,
            resource: None,
            dry_run: None,
            partial_success: None,
        }
    }

    pub fn entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

impl TailRequest {
    pub fn from_resource_name<T: Into<String>>(r: T) -> TailRequest {
        Self {
            resource_names: vec![r.into()],
            filter: None,
            buffer_window: None,
        }
    }
}

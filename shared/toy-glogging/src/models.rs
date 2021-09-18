use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Severity {
    DEFAULT,
    INFO,
    DEBUG,
    ERROR,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "logName")]
    log_name: String,
    resource: Resource,
    timestamp: Option<String>,
    #[serde(rename = "receiveTimestamp")]
    receive_timestamp: Option<String>,
    severity: Option<Severity>,
    #[serde(rename = "insertId")]
    insert_id: Option<String>,
    labels: Option<HashMap<String, String>>,
    operation: Option<Operation>,
    #[serde(rename = "jsonPayload")]
    json_payload: Option<HashMap<String, String>>,
}

pub struct EntryBuilder {
    e: Entry,

    json_payload: HashMap<String, String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "type")]
    tp: String,
    labels: Option<HashMap<String, String>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    id: Option<String>,
    producer: Option<String>,
    first: Option<bool>,
    last: Option<bool>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    error: ErrorInfo,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct ErrorInfo {
    code: u32,
    message: String,
    status: String,
    details: Vec<ErrorDetail>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "@type")]
    tp: String,
    #[serde(rename = "fieldViolations")]
    field_violations: Vec<FieldViolation>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct FieldViolation {
    description: String,
}

///////////////////////////////
// List
///////////////////////////////

#[derive(Clone, Debug, Serialize)]
pub struct ListRequest {
    #[serde(rename = "resourceNames")]
    resource_names: Vec<String>,
    filter: Option<String>,
    #[serde(rename = "orderBy")]
    order_by: Option<String>,
    #[serde(rename = "pageSize")]
    page_size: Option<u32>,
    #[serde(rename = "pageToken")]
    page_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    entries: Vec<Entry>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

///////////////////////////////
// Write
///////////////////////////////

#[derive(Clone, Debug, Serialize)]
pub struct WriteRequest {
    #[serde(rename = "logName")]
    log_name: Option<String>,
    resource: Option<Resource>,
    labels: Option<HashMap<String, String>>,
    entries: Vec<Entry>,
    #[serde(rename = "partialSuccess")]
    partial_success: Option<bool>,
    #[serde(rename = "dryRun")]
    dry_run: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WriteResponse;

///////////////////////////////
// Tail
///////////////////////////////
#[derive(Clone, Debug, Serialize)]
pub struct TailRequest {
    #[serde(rename = "resourceNames")]
    resource_names: Vec<String>,
    filter: Option<String>,
    #[serde(rename = "bufferWindow")]
    buffer_window: Option<TailDuration>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct TailDuration {
    seconds: u64,
    nanos: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TailResponse {
    entries: Vec<Entry>,
    #[serde(rename = "suppressionInfo")]
    suppression_info: SuppressionInfo,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct SuppressionInfo {
    reason: SuppressionReason,
    #[serde(rename = "suppressedCount")]
    suppressed_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum SuppressionReason {
    #[serde(rename = "REASON_UNSPECIFIED")]
    ReasonUnspecified,
    #[serde(rename = "RATE_LIMIT")]
    RateLimit,
    #[serde(rename = "NOT_CONSUMED")]
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

impl Operation {
    pub fn is_first(&self) -> bool {
        self.first.is_some() && self.first.unwrap()
    }

    pub fn is_last(&self) -> bool {
        self.last.is_some() && self.last.unwrap()
    }

    pub fn first<T: Into<String>>(id: T) -> Operation {
        Operation {
            id: Some(id.into()),
            producer: None,
            first: Some(true),
            last: None,
        }
    }

    pub fn last<T: Into<String>>(id: T) -> Operation {
        Operation {
            id: Some(id.into()),
            producer: None,
            first: None,
            last: Some(true),
        }
    }

    pub fn with_producer<T: Into<String>>(self, producer: T) -> Operation {
        Operation {
            producer: Some(producer.into()),
            ..self
        }
    }
}

impl Entry {
    /// Get timestamp.
    pub fn timestamp(&self) -> Option<&String> {
        self.timestamp.as_ref()
    }

    /// Get json payload.
    pub fn json_payload(&self) -> Option<&HashMap<String, String>> {
        self.json_payload.as_ref()
    }

    /// Convert to `String` json payload.
    pub fn json_payload_raw(&self) -> Option<String> {
        match self.json_payload.as_ref() {
            Some(v) => toy_pack_json::pack_to_string(v).map_or_else(|_| None, Some),
            None => None,
        }
    }

    /// Get label.
    pub fn label<K: AsRef<str>>(&self, k: K) -> Option<&String> {
        match self.labels {
            Some(ref map) => map.get(k.as_ref()),
            None => None,
        }
    }

    /// Get operation.
    pub fn operation(&self) -> Option<&Operation> {
        self.operation.as_ref()
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
                operation: None,
                json_payload: None,
            },
            json_payload: HashMap::new(),
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

    /// replace json payload.
    /// clear builing 'kv'.
    pub fn json_payload(mut self, json: HashMap<String, String>) -> EntryBuilder {
        self.e.json_payload = Some(json);
        self.json_payload.clear();
        self
    }

    /// push json payload
    pub fn kv<K: Into<String>, V: Into<String>>(mut self, k: K, v: V) -> EntryBuilder {
        self.json_payload.insert(k.into(), v.into());
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

    pub fn opelation(mut self, v: Operation) -> EntryBuilder {
        self.e.operation = Some(v);
        self
    }

    pub fn opelation_opt(self, v: Option<Operation>) -> EntryBuilder {
        match v {
            Some(v) => self.opelation(v),
            None => self,
        }
    }

    /// build `Entry`
    pub fn build(mut self) -> Entry {
        if !self.json_payload.is_empty() {
            self.e.json_payload = Some(self.json_payload.clone());
        }
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

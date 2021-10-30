use crate::kv::{
    encode, DeleteRangeRequest, DeleteRangeResponse, PutRequest, PutResponse, RangeRequest,
    RangeResponse, ResponseHeader,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompareResult {
    EQUAL,
    GREATER,
    LESS,
    #[serde(rename = "NOT_EQUAL")]
    NotEqual,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompareTarget {
    VERSION,
    CREATE,
    MOD,
    VALUE,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Compare {
    key: String,
    result: Option<CompareResult>,
    target: Option<CompareTarget>,

    version: Option<String>,
    create_revision: Option<String>,
    mod_revision: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct RequestOp {
    request_range: Option<RangeRequest>,
    request_put: Option<PutRequest>,
    request_delete_range: Option<DeleteRangeRequest>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ResponseOp {
    response_range: Option<RangeResponse>,
    response_put: Option<PutResponse>,
    response_delete_range: Option<DeleteRangeResponse>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct TxnRequest {
    compare: Vec<Compare>,
    success: Option<Vec<RequestOp>>,
    failure: Option<Vec<RequestOp>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TxnResponse {
    header: ResponseHeader,
    succeeded: Option<bool>,
    #[serde(default)]
    responses: Vec<ResponseOp>,
}

impl Compare {
    pub fn not_exists(key: &str) -> Compare {
        let encoded_key = encode(key);
        Compare {
            key: encoded_key,
            result: None,
            target: Some(CompareTarget::CREATE),
            version: None,
            create_revision: None,
            mod_revision: Some("0".to_string()),
        }
    }

    pub fn with(key: &str, result: CompareResult, target: CompareTarget, rev: String) -> Compare {
        let encoded_key = encode(key);
        let (version, create_revision, mod_revision) = {
            match target {
                CompareTarget::VERSION => (Some(rev), None, None),
                CompareTarget::CREATE => (None, Some(rev), None),
                CompareTarget::MOD => (None, None, Some(rev)),
                CompareTarget::VALUE => (None, None, None),
            }
        };
        Compare {
            key: encoded_key,
            result: Some(result),
            target: Some(target),
            version,
            create_revision,
            mod_revision,
        }
    }
}

impl RequestOp {
    pub fn range(req: RangeRequest) -> RequestOp {
        RequestOp {
            request_range: Some(req),
            request_put: None,
            request_delete_range: None,
        }
    }

    pub fn put(req: PutRequest) -> RequestOp {
        RequestOp {
            request_range: None,
            request_put: Some(req),
            request_delete_range: None,
        }
    }

    pub fn delete(req: DeleteRangeRequest) -> RequestOp {
        RequestOp {
            request_range: None,
            request_put: None,
            request_delete_range: Some(req),
        }
    }
}

impl TxnRequest {
    pub fn with(compare: Compare, success: RequestOp) -> TxnRequest {
        TxnRequest {
            compare: vec![compare],
            success: Some(vec![success]),
            failure: None,
        }
    }
}

impl TxnResponse {
    pub fn is_success(&self) -> bool {
        self.succeeded.is_some() && self.succeeded.unwrap()
    }
}

impl Default for CompareResult {
    fn default() -> Self {
        CompareResult::EQUAL
    }
}

impl Default for CompareTarget {
    fn default() -> Self {
        CompareTarget::CREATE
    }
}

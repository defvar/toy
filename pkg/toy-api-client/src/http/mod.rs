mod graph;
mod service;
mod supervisor;
mod task;

pub use graph::HttpGraphClient;
pub use service::HttpServiceClient;
pub use supervisor::HttpSupervisorClient;
pub use task::HttpTaskClient;

use crate::client::ApiClient;
use crate::error::ApiClientError;

use toy_api::common::Format;
use toy_h::impl_reqwest::ReqwestClient;
use toy_pack::ser::Serializable;
use toy_pack::Pack;
use toy_pack_urlencoded::QueryParseError;

#[derive(Debug, Clone)]
pub struct HttpApiClient {
    graph: HttpGraphClient<ReqwestClient>,
    task: HttpTaskClient<ReqwestClient>,
    sv: HttpSupervisorClient<ReqwestClient>,
    service: HttpServiceClient<ReqwestClient>,
}

impl HttpApiClient {
    pub fn new<P: AsRef<str>>(root: P) -> Result<Self, ApiClientError> {
        let client = ReqwestClient::new()?;
        Ok(Self {
            graph: HttpGraphClient::new(root.as_ref(), client.clone()),
            task: HttpTaskClient::new(root.as_ref(), client.clone()),
            sv: HttpSupervisorClient::new(root.as_ref(), client.clone()),
            service: HttpServiceClient::new(root.as_ref(), client.clone()),
        })
    }

    pub fn from<P: AsRef<str>>(root: P, inner: ReqwestClient) -> Result<Self, ApiClientError> {
        Ok(Self {
            graph: HttpGraphClient::new(root.as_ref(), inner.clone()),
            task: HttpTaskClient::new(root.as_ref(), inner.clone()),
            sv: HttpSupervisorClient::new(root.as_ref(), inner.clone()),
            service: HttpServiceClient::new(root.as_ref(), inner.clone()),
        })
    }
}

impl ApiClient for HttpApiClient {
    type Graph = HttpGraphClient<ReqwestClient>;
    type Task = HttpTaskClient<ReqwestClient>;
    type Supervisor = HttpSupervisorClient<ReqwestClient>;
    type Service = HttpServiceClient<ReqwestClient>;

    fn graph(&self) -> &Self::Graph {
        &self.graph
    }

    fn task(&self) -> &Self::Task {
        &self.task
    }

    fn supervisor(&self) -> &Self::Supervisor {
        &self.sv
    }

    fn service(&self) -> &Self::Service {
        &self.service
    }
}

pub(crate) fn common_headers(format: Option<Format>) -> toy_h::HeaderMap {
    use toy_h::{header::HeaderValue, header::CONTENT_TYPE, HeaderMap};

    let mut headers = HeaderMap::new();

    let v = match format.unwrap_or(Format::MessagePack) {
        Format::Json => HeaderValue::from_static("application/json"),
        Format::MessagePack => HeaderValue::from_static("application/x-msgpack"),
        Format::Yaml => HeaderValue::from_static("application/yaml"),
    };
    headers.insert(CONTENT_TYPE, v);

    headers
}

pub(crate) fn prepare_query<T>(p: &T) -> Result<String, QueryParseError>
where
    T: Serializable,
{
    #[derive(Pack)]
    struct DefaultFormat {
        format: Format,
    }

    let mut q: String = toy_pack_urlencoded::pack_to_string(p)?;
    if !q.contains("format") {
        if q.contains("=") {
            q.push('&');
        }
        let q2 = toy_pack_urlencoded::pack_to_string(&DefaultFormat {
            format: Format::MessagePack,
        })?;
        q.push_str(&q2);
    }
    Ok(q)
}

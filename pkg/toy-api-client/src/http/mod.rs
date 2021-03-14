mod graph;
mod supervisor;
mod task;

pub use graph::HttpGraphClient;
pub use supervisor::HttpSupervisorClient;
pub use task::HttpTaskClient;

use crate::client::ApiClient;
use crate::error::ApiClientError;
use toy_h::impl_reqwest::ReqwestClient;

#[derive(Debug)]
pub struct HttpApiClient {
    g: HttpGraphClient<ReqwestClient>,
    t: HttpTaskClient<ReqwestClient>,
    s: HttpSupervisorClient<ReqwestClient>,
}

impl HttpApiClient {
    pub fn new<P: AsRef<str>>(root: P) -> Result<Self, ApiClientError> {
        let client = ReqwestClient::new()?;
        Ok(Self {
            g: HttpGraphClient::new(root.as_ref(), client.clone()),
            t: HttpTaskClient::new(root.as_ref(), client.clone()),
            s: HttpSupervisorClient::new(root.as_ref(), client.clone()),
        })
    }

    pub fn from<P: AsRef<str>>(root: P, inner: ReqwestClient) -> Result<Self, ApiClientError> {
        Ok(Self {
            g: HttpGraphClient::new(root.as_ref(), inner.clone()),
            t: HttpTaskClient::new(root.as_ref(), inner.clone()),
            s: HttpSupervisorClient::new(root.as_ref(), inner.clone()),
        })
    }
}

impl ApiClient for HttpApiClient {
    type Graph = HttpGraphClient<ReqwestClient>;
    type Task = HttpTaskClient<ReqwestClient>;
    type Supervisor = HttpSupervisorClient<ReqwestClient>;

    fn graph(&self) -> &Self::Graph {
        &self.g
    }

    fn task(&self) -> &Self::Task {
        &self.t
    }

    fn supervisor(&self) -> &Self::Supervisor {
        &self.s
    }
}

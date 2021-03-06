use crate::api::{graphs, services, supervisors, tasks};
use crate::auth::auth_filter;
use crate::config::ServerConfig;
use crate::graph::store::GraphStore;
use crate::reject_handler::handle_rejection;
use crate::services::store::ServiceStore;
use crate::supervisors::store::SupervisorStore;
use crate::task::store::{TaskLogStore, TaskStore};
use std::net::SocketAddr;
use toy_h::HttpClient;
use warp::http::Method;
use warp::{Filter, Reply};

/// toy-api Server.
#[derive(Debug)]
pub struct Server<Config, Http> {
    client: Option<Http>,
    config: Config,
}

impl<Config, Http> Server<Config, Http>
where
    Config: ServerConfig<Http>,
    Http: HttpClient + 'static,
{
    pub fn new(config: Config) -> Server<Config, Http> {
        Server {
            client: None,
            config,
        }
    }

    /// Use http client.
    pub fn with_client(mut self, client: Http) -> Self {
        self.client = Some(client);
        self
    }

    /// Run this `Server` forever on the current thread, specified routes.
    pub async fn run_with_routes<F>(&self, addr: impl Into<SocketAddr> + 'static, routes: F)
    where
        F: Filter + Clone + Send + Sync + 'static,
        F::Extract: Reply,
    {
        let (addr, server) = warp::serve(routes)
            .tls()
            .cert_path(self.config.cert_path())
            .key_path(self.config.key_path())
            .bind_ephemeral(addr);
        tracing::info!("listening on https://{}", addr);
        server.await
    }

    /// Run this `Server` forever on the current thread, default routes.
    pub async fn run(&self, addr: impl Into<SocketAddr> + 'static) {
        let client = match self.client {
            Some(ref c) => c.clone(),
            None => {
                tracing::error!("http client not build.");
                return;
            }
        };
        let mut graph_store = self.config.graph_store();
        let mut log_store = self.config.task_log_store();
        let mut task_store = self.config.task_store();
        let mut supervisor_store = self.config.supervisor_store();
        let mut service_store = self.config.service_store();
        if let Err(e) = graph_store.establish(client.clone()) {
            tracing::error!("graph store connection failed. error:{:?}", e);
            return;
        };
        if let Err(e) = log_store.establish(client.clone()) {
            tracing::error!("log store connection failed. error:{:?}", e);
            return;
        };
        if let Err(e) = task_store.establish(client.clone()) {
            tracing::error!("task store connection failed. error:{:?}", e);
            return;
        };
        if let Err(e) = supervisor_store.establish(client.clone()) {
            tracing::error!("supervisor store connection failed. error:{:?}", e);
            return;
        };
        if let Err(e) = service_store.establish(client.clone()) {
            tracing::error!("service store connection failed. error:{:?}", e);
            return;
        };
        let routes = auth_filter(self.config.auth(), client)
            .and(
                graphs(graph_store)
                    .or(tasks(log_store, task_store))
                    .or(supervisors(supervisor_store))
                    .or(services(service_store)),
            )
            .map(|_, r| r)
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_headers(vec!["authorization"])
                    .allow_methods(&[
                        Method::GET,
                        Method::OPTIONS,
                        Method::POST,
                        Method::DELETE,
                        Method::PUT,
                    ]),
            )
            .with(warp::trace::request())
            .recover(handle_rejection);

        self.run_with_routes(addr, routes).await
    }
}

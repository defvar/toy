use crate::api::{graphs, services, tasks};
use crate::auth::auth_filter;
use crate::config::ServerConfig;
use crate::graph::store::GraphStore;
use crate::task::store::TaskLogStore;
use std::net::SocketAddr;
use toy::core::error::ServiceError;
use toy::core::mpsc::{Incoming, Outgoing};
use toy::supervisor::{Request, SystemMessage};
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
    pub async fn run_with_routes<F>(
        &self,
        addr: impl Into<SocketAddr> + 'static,
        mut rx: Incoming<SystemMessage, ServiceError>,
        routes: F,
    ) where
        F: Filter + Clone + Send + Sync + 'static,
        F::Extract: Reply,
    {
        let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(addr, async move {
            while let Some(r) = rx.next().await {
                match r {
                    Ok(r) => match r {
                        SystemMessage::Shutdown => {
                            tracing::info!("shutdown api server because stoped supervisor.");
                            break;
                        }
                    },
                    Err(e) => tracing::error!("error receive system message. error:{:?}", e),
                }
            }
        });
        tracing::info!("listening on http://{}", addr);
        server.await
    }

    /// Run this `Server` forever on the current thread, default routes.
    pub async fn run(
        &self,
        addr: impl Into<SocketAddr> + 'static,
        tx: Outgoing<Request, ServiceError>,
        rx: Incoming<SystemMessage, ServiceError>,
    ) {
        let client = match self.client {
            Some(ref c) => c.clone(),
            None => {
                tracing::error!("http client not build.");
                return;
            }
        };
        let mut graph_store = self.config.graph_store();
        let mut log_store = self.config.task_log_store();
        if let Err(e) = graph_store.establish(client.clone()) {
            tracing::error!("graph store connection failed. error:{:?}", e);
            return;
        };
        if let Err(e) = log_store.establish(client.clone()) {
            tracing::error!("log store connection failed. error:{:?}", e);
            return;
        };
        let routes = auth_filter(self.config.auth(), client)
            .and(
                graphs(graph_store, tx.clone())
                    .or(services(tx.clone()))
                    .or(tasks(log_store, tx.clone())),
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
            .with(warp::trace::request());

        self.run_with_routes(addr, rx, routes).await
    }
}

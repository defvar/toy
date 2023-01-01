use crate::context::SupervisorContext;
use crate::http::handler;
use crate::SupervisorConfig;
use std::net::SocketAddr;
use toy_api_client::ApiClient;
use toy_api_http_common::axum::routing::{get, post, put};
use toy_api_http_common::axum::Router;
use toy_api_http_common::axum_server::tls_rustls::RustlsConfig;
use toy_core::mpsc::Incoming;

pub struct Server<C> {
    ctx: SupervisorContext<C>,
}

impl<C> Server<C>
where
    C: ApiClient + Clone + Send + Sync + 'static,
{
    pub fn new(ctx: SupervisorContext<C>) -> Server<C> {
        Server { ctx }
    }

    /// Run this `Server` forever on the current thread.
    pub async fn run(
        self,
        addr: impl Into<SocketAddr> + 'static,
        config: impl SupervisorConfig,
        mut _shutdown_receiver: Incoming<()>,
    ) {
        let config = RustlsConfig::from_pem_file(config.cert_path(), config.key_path())
            .await
            .unwrap();

        let app = Router::new()
            .route("/", get(handler::index))
            .route("/status", get(handler::status))
            .route("/services", get(handler::services))
            .route("/tasks", post(handler::tasks))
            .route("/event_buffers", get(handler::event_buffers))
            .route("/metrics", get(handler::metrics))
            .route("/shutdown", put(handler::shutdown))
            .with_state(self.ctx);

        let addr = addr.into();
        tracing::info!("listening on https://{}", addr);
        toy_api_http_common::axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

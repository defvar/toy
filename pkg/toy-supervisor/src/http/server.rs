use crate::http::filter::filters;
use crate::supervisor::SupervisorContext;
use crate::{SupervisorConfig, SupervisorError};
use std::net::SocketAddr;
use toy_api_client::ApiClient;
use toy_api_http_common::warp;
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
        mut shutdown_receiver: Incoming<(), SupervisorError>,
    ) {
        let (addr, server) = warp::serve(filters(self.ctx))
            .tls()
            .cert_path(config.cert_path())
            .key_path(config.key_path())
            .bind_with_graceful_shutdown(addr, async move {
                shutdown_receiver.next().await;
            });
        tracing::info!("listening on https://{}", addr);
        server.await
    }
}

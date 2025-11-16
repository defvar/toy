use crate::api::{graph, metrics, rbac, services, supervisors, task};
use crate::config::ServerConfig;
use crate::context::{ServerState, WrappedState};
use crate::store::kv::KvStore;
use crate::store::metrics::MetricsStore;
use crate::store::task_event::TaskEventStore;
use std::net::SocketAddr;
use toy_api_http_common::axum::routing::{get, post};
use toy_api_http_common::axum::Router;
use toy_api_http_common::axum_server::tls_rustls::RustlsConfig;
use toy_api_http_common::cors::CorsLayer;
use toy_api_http_common::trace::TraceLayer;
use toy_h::HttpClient;

/// toy-api Server.
#[derive(Debug)]
pub struct Server<Config> {
    config: Config,
}

impl<Config> Server<Config>
where
    Config: ServerConfig + Sync + Send + Clone + 'static,
{
    pub fn new(config: Config) -> Server<Config> {
        Server { config }
    }

    /// Run this `Server` forever on the current thread, default routes.
    pub async fn run(
        self,
        mut state: impl ServerState<Client = impl HttpClient + 'static> + 'static,
        addr: impl Into<SocketAddr> + 'static,
    ) {
        let c = state.client().clone();
        if let Err(e) = state.kv_store_mut().establish(c.clone()) {
            tracing::error!("kv store connection failed. error:{}", e);
        }
        if let Err(e) = state.task_event_store_mut().establish(c.clone()) {
            tracing::error!("task log store connection failed. error:{}", e);
        }
        if let Err(e) = state.metrics_store_mut().establish(c.clone()) {
            tracing::error!("metrics store connection failed. error:{}", e);
        }

        if let Err(e) =
            crate::initializer::initialize(&self.config, state.kv_store(), state.client().clone())
                .await
        {
            tracing::error!("{}", e);
            return;
        }

        let tls = RustlsConfig::from_pem_file(self.config.cert_path(), self.config.key_path())
            .await
            .unwrap();

        let app = Router::new()
            .route(
                "/supervisors/{key}",
                get(supervisors::find)
                    .put(supervisors::put)
                    .delete(supervisors::delete),
            )
            .route("/supervisors", get(supervisors::list))
            .route("/supervisors/{key}/beat", post(supervisors::beat))
            .route(
                "/services/{key}",
                get(services::find)
                    .put(services::put)
                    .delete(services::delete),
            )
            .route("/services", get(services::list))
            .route(
                "/graphs/{key}",
                get(graph::find).put(graph::put).delete(graph::delete),
            )
            .route("/graphs", get(graph::list))
            .route(
                "/rbac/roles/{key}",
                get(rbac::role::find)
                    .put(rbac::role::put)
                    .delete(rbac::role::delete),
            )
            .route("/rbac/roles", get(rbac::role::list))
            .route(
                "/rbac/roleBindings/{key}",
                get(rbac::role_binding::find)
                    .put(rbac::role_binding::put)
                    .delete(rbac::role_binding::delete),
            )
            .route("/rbac/roleBindings", get(rbac::role_binding::list))
            .route("/tasks", get(task::list_task).post(task::post))
            .route("/tasks/{key}", get(task::find))
            .route("/tasks/{key}/finish", post(task::finish))
            .route(
                "/tasks/events",
                get(task::list_task_event).post(task::post_task_event),
            )
            .route("/metrics", post(metrics::post))
            .layer(CorsLayer::very_permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(WrappedState::new(state));

        let addr = addr.into();
        tracing::info!("listening on https://{}", addr);
        toy_api_http_common::axum_server::bind_rustls(addr, tls)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

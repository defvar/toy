#![feature(error_generic_member_access)]

use crate::error::Error;
use crate::opts::*;
use clap::Parser;
use std::net::SocketAddr;
use toy::api_server::authentication::CommonAuths;
use toy::api_server::context::ServerState;
use toy::api_server::ServerConfig;
use toy_api_auth_jwt::JWTAuth;
use toy_api_store_etcd::EtcdStore;
use toy_api_store_pg::PgStore;
use toy_h::impl_reqwest::ReqwestClient;
use toy_tracing::{LogGuard, CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT};

mod error;
mod opts;

#[derive(Clone)]
struct ToyConfig {
    cert_path: String,
    key_path: String,
    pub_path: String,
    tls_secret_key: String,
    dispatch_interval_mills: u64,
    clean_actor_interval_mills: u64,
}

#[derive(Clone)]
struct ToyState {
    client: ReqwestClient,
    auth: CommonAuths<JWTAuth, JWTAuth>,
    kv_store: EtcdStore<ReqwestClient>,
    task_log_store: PgStore,
    metrics_store: PgStore,
}

impl ServerConfig for ToyConfig {
    fn cert_path(&self) -> String {
        self.cert_path.clone()
    }

    fn key_path(&self) -> String {
        self.key_path.clone()
    }

    fn pub_path(&self) -> String {
        self.pub_path.clone()
    }

    fn tls_secret_key(&self) -> String {
        self.tls_secret_key.clone()
    }

    fn dispatch_interval_mills(&self) -> u64 {
        self.dispatch_interval_mills
    }

    fn clean_actor_interval_mills(&self) -> u64 {
        self.clean_actor_interval_mills
    }
}

impl ServerState for ToyState {
    type Client = ReqwestClient;
    type Auth = CommonAuths<JWTAuth, JWTAuth>;
    type KvStore = EtcdStore<ReqwestClient>;
    type TaskEventStore = PgStore;
    type MetricsStore = PgStore;

    fn client(&self) -> &Self::Client {
        &self.client
    }

    fn auth(&self) -> &Self::Auth {
        &self.auth
    }

    fn kv_store(&self) -> &Self::KvStore {
        &self.kv_store
    }

    fn kv_store_mut(&mut self) -> &mut Self::KvStore {
        &mut self.kv_store
    }

    fn task_event_store(&self) -> &Self::TaskEventStore {
        &self.task_log_store
    }

    fn task_event_store_mut(&mut self) -> &mut Self::TaskEventStore {
        &mut self.task_log_store
    }

    fn metrics_store(&self) -> &Self::MetricsStore {
        &self.metrics_store
    }

    fn metrics_store_mut(&mut self) -> &mut Self::MetricsStore {
        &mut self.metrics_store
    }
}

fn main() {
    if let Err(e) = go() {
        tracing::error!("{:?}", e);
    }
}

fn go() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let opts: Opts = Opts::parse();

    let host = opts.host;
    let port = opts.port;

    let host = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("invalid IP Address.");

    let (_guard, tracing_addr) = initialize_log(&opts.log)?;

    let mut api_rt = toy_rt::RuntimeBuilder::new()
        .thread_name(opts.thread_name_prefix)
        .worker_threads(opts.worker)
        .build()
        .unwrap();

    let client = ReqwestClient::new().unwrap();
    let config = ToyConfig {
        cert_path: opts.cert_path.to_string(),
        key_path: opts.key_path.to_string(),
        pub_path: opts.pub_path.to_string(),
        tls_secret_key: opts.tls_secret_key.to_string(),
        dispatch_interval_mills: opts.dispatch_interval_mills,
        clean_actor_interval_mills: opts.clean_actor_interval_mills,
    };
    let state = ToyState {
        client: client.clone(),
        auth: CommonAuths::new(JWTAuth::new(), JWTAuth::new()),
        kv_store: EtcdStore::new(),
        task_log_store: PgStore::new(),
        metrics_store: PgStore::new(),
    };

    let server = toy::api_server::Server::new(config);

    api_rt.block_on(async move {
        tracing::info!("start api-server :{}", host);
        tracing::info!("tokio tracing console :{}", tracing_addr);
        let _ = server.run(state, host).await;
    });

    Ok(())
}

fn initialize_log(opt: &LogOption) -> Result<(LogGuard, SocketAddr), Error> {
    let addr = match (&opt.tokio_console_host, &opt.tokio_console_port) {
        (Some(host), Some(port)) => format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .expect("invalid IP Address."),
        (Some(host), None) => format!("{}:{}", host, CONSOLE_DEFAULT_PORT)
            .parse::<SocketAddr>()
            .expect("invalid IP Address."),
        (None, Some(port)) => format!("{}:{}", CONSOLE_DEFAULT_IP, port)
            .parse::<SocketAddr>()
            .expect("invalid IP Address."),
        _ => SocketAddr::new(CONSOLE_DEFAULT_IP, CONSOLE_DEFAULT_PORT),
    };

    let format = match opt.format {
        LogFormat::Text => toy_tracing::LogFormat::Text,
        LogFormat::Json => toy_tracing::LogFormat::Json,
    };

    let tracing_opt = toy_tracing::LogOption::default()
        .with_ansi(opt.ansi)
        .with_format(format)
        .with_tokio_console_addr(addr);

    match opt.log {
        Some(ref path) => match (path.as_path().parent(), path.as_path().file_name()) {
            (Some(dir), Some(prefix)) => toy_tracing::file_with(dir, prefix, tracing_opt)
                .map_err(|x| x.into())
                .map(|g| (g, addr)),
            _ => Err(Error::invalid_log_path()),
        },
        None => toy_tracing::console_with(tracing_opt)
            .map_err(|x| x.into())
            .map(|g| (g, addr)),
    }
}

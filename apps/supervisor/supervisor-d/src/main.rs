use crate::error::Error;
use clap::Clap;
use std::fs::File;
use std::io::Read;
use toy::api_client::http::HttpApiClient;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy_api::authentication::Claims;
use toy_jwt::Algorithm;
use tracing_subscriber::fmt::format::FmtSpan;

mod error;

#[derive(Clap, Debug)]
#[clap(version = "0.1")]
struct Opts {
    name: String,
    #[clap(short, long, default_value = "4")]
    worker: usize,
    #[clap(short, long, default_value = "supervisor")]
    thread_name_prefix: String,
    #[clap(short, long, default_value = "https://localhost:3030")]
    api_root: String,
    #[clap(short, long)]
    user: Option<String>,
    #[clap(short, long)]
    credential: Option<String>,
}

fn main() {
    if let Err(e) = go() {
        tracing::error!("{:?}", e);
    }
}

fn go() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    dotenv::dotenv().ok();

    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(time)
        .init();

    let user = match opts.user {
        Some(ref p) => p.to_owned(),
        None => std::env::var("TOY_API_CLIENT_USER")?,
    };

    let token = get_credential(&user, opts.credential.as_ref())?;
    let auth = toy::api_client::auth::Auth::with_bearer_token(user, token);

    tracing::info!("start supervisor for config:{:?}", opts);

    let mut rt = toy_rt::RuntimeBuilder::new()
        .thread_name(format!("{}-{}", opts.thread_name_prefix, opts.name))
        .worker_threads(opts.worker)
        .build()
        .unwrap();

    let app = app(toy_plugin_commons::load());
    let client = HttpApiClient::new(&opts.api_root, auth).unwrap();
    let (sv, _tx, _rx) = toy::supervisor::spawn(opts.name, ExecutorFactory, app, client);

    rt.block_on(async {
        let _ = sv.run().await;
    });

    Ok(())
}

fn get_credential(user: &str, path_string: Option<&String>) -> Result<String, Error> {
    let p = match path_string {
        Some(p) => p.to_owned(),
        None => std::env::var("TOY_API_CLIENT_CREDENTIAL")?,
    };

    let mut f = File::open(p)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let claims = Claims::new(user);

    let token = toy_jwt::encode::from_rsa_pem(
        &claims,
        Algorithm::RS256,
        Some("__TLS_SECRET_KID__".to_owned()),
        buffer.as_bytes(),
    )?;

    Ok(token)
}

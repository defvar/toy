#![feature(backtrace)]

use crate::error::Error;
use clap::Clap;
use std::fs::File;
use std::io::Read;
use toy::api_client::http::HttpApiClient;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy_api::authentication::Claims;
use toy_jwt::Algorithm;

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
    #[clap(short, long, env = "TOY_API_CLIENT_USER")]
    user: String,
    #[clap(short, long, env = "TOY_API_CLIENT_CREDENTIAL")]
    credential: String,
    #[clap(short, long, env = "TOY_API_CLIENT_KID")]
    kid: String,
}

fn main() {
    if let Err(e) = go() {
        tracing::error!("{:?}", e);
    }
}

fn go() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    dotenv::dotenv().ok();

    //let g = toy_tracing::tcp("127.0.0.1:6060")?;
    toy_tracing::console();

    let token = get_credential(&opts.user, &opts.kid, &opts.credential)
        .map_err(|e| Error::read_credential_error(e))?;
    let auth = toy::api_client::auth::Auth::with_bearer_token(&opts.user, &token);

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

fn get_credential(user: &str, kid: &str, path_string: &str) -> Result<String, Error> {
    let mut f = File::open(path_string)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let claims = Claims::new(user);

    let token =
        toy_jwt::encode::from_rsa_pem(&claims, Algorithm::RS256, Some(kid.to_owned()), &buffer)?;

    Ok(token)
}

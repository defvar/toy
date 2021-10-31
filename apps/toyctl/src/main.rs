use crate::error::Error;
use crate::opts::*;

use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::authentication::Claims;
use toy_jwt::Algorithm;
use tracing_subscriber::fmt::format::FmtSpan;

mod commands;
mod error;
mod opts;
mod output;

fn main() {
    let opts: Opts = Opts::parse();

    let time = tracing_subscriber::fmt::time::ChronoUtc::rfc3339();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(time)
        .init();

    let mut rt = toy_rt::RuntimeBuilder::new()
        .thread_name("toyctl")
        .worker_threads(2)
        .build()
        .unwrap();

    let token = get_credential(&opts.config.user, &opts.config.kid, &opts.config.credential);
    if let Err(e) = token {
        tracing::error!("{:?}", e);
        return;
    }

    let auth = toy::api_client::auth::Auth::with_bearer_token(&opts.config.user, &token.unwrap());
    let client = HttpApiClient::new(&opts.config.api_root, auth).unwrap();

    rt.block_on(async {
        let w = std::io::BufWriter::new(std::io::stdout());
        let r = go(opts.c, client, w).await;
        if let Err(e) = r {
            tracing::error!("{:?}", e);
        }
    })
}

async fn go<W>(c: Command, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    match c {
        Command::List(c) => commands::list::execute(c, client, writer).await,
        Command::Put(c) => commands::put::execute(c, client, writer).await,
    }
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

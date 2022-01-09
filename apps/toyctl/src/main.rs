use crate::error::Error;
use crate::opts::*;

use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use toy::api_client::http::HttpApiClient;
use toy::api_client::toy_api::authentication::Claims;
use toy_jwt::Algorithm;
use toy_tracing::LogGuard;

mod commands;
mod error;
mod opts;
mod output;

fn main() {
    dotenv::dotenv().ok();

    let opts: Opts = Opts::parse();

    let guard = initialize_log(&opts.config.log);
    if let Err(e) = guard {
        eprintln!("{:?}", e);
        return;
    }
    let _guard = guard.unwrap();

    let mut rt = toy_rt::RuntimeBuilder::new()
        .thread_name("toyctl")
        .worker_threads(2)
        .build()
        .unwrap();

    let token = get_credential(&opts.config.user, &opts.config.kid, &opts.config.credential);
    if let Err(e) = token {
        tracing::error!(err = %e);
        eprintln!("{}", e.to_string());
        return;
    }

    let auth = toy::api_client::auth::Auth::with_bearer_token(&opts.config.user, &token.unwrap());
    let client = HttpApiClient::new(&opts.config.api_root, auth).unwrap();

    rt.block_on(async {
        let w = std::io::BufWriter::new(std::io::stdout());
        let r = go(opts.c, client, w).await;
        if let Err(e) = r {
            tracing::error!(err = %e);
            eprintln!("{}", e.to_string());
        }
    })
}

async fn go<W>(c: Command, client: HttpApiClient, writer: W) -> Result<(), Error>
where
    W: Write,
{
    match c {
        Command::Find(c) => commands::find::execute(c, client, writer).await,
        Command::List(c) => commands::list::execute(c, client, writer).await,
        Command::Put(c) => commands::put::execute(c, client, writer).await,
        Command::Post(c) => commands::post::execute(c, client, writer).await,
    }
}

fn initialize_log(opt: &LogOption) -> Result<LogGuard, Error> {
    let path = match &opt.log {
        Some(v) => v.clone(),
        None => dirs::home_dir()
            .map(|h| h.join("toy").join("toyctl.log"))
            .unwrap(),
    };

    match (path.as_path().parent(), path.as_path().file_name()) {
        (Some(dir), Some(prefix)) => {
            toy_tracing::file(dir, prefix, toy_tracing::LogRotation::Never).map_err(|x| x.into())
        }
        _ => Err(Error::invalid_log_path()),
    }
}

fn get_credential(user: &str, kid: &str, path_string: &str) -> Result<String, Error> {
    fn get_credential0(user: &str, kid: &str, path_string: &str) -> Result<String, Error> {
        let mut f = File::open(path_string)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let claims = Claims::new(user);

        let token = toy_jwt::encode::from_rsa_pem(
            &claims,
            Algorithm::RS256,
            Some(kid.to_owned()),
            &buffer,
        )?;

        Ok(token)
    }

    get_credential0(user, kid, path_string)
        .map_err(|e| Error::read_credential_error(path_string, e.into()))
}

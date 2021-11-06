#![feature(backtrace)]

use crate::error::Error;
use crate::opts::*;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toy::api_client::http::HttpApiClient;
use toy::core::prelude::*;
use toy::executor::ExecutorFactory;
use toy_api::authentication::Claims;
use toy_jwt::Algorithm;
use toy_tracing::LogGuard;

mod error;
mod opts;

fn main() {
    if let Err(e) = go() {
        tracing::error!("{:?}", e);
    }
}

fn go() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let opts: Opts = Opts::parse();

    let app = app(toy_plugin_commons::map::all())
        .with(toy_plugin_commons::collect::all())
        .with(toy_plugin_commons::fanout::all())
        .with(toy_plugin_commons::stdio::all())
        .with(toy_plugin_commons::file::all())
        .with(toy_plugin_commons::tcp::all())
        .with(toy_plugin_commons::timer::all())
        .with(toy_plugin_commons::sort::all())
        .build();

    let thread_name = format!(
        "{}-{}",
        opts.thread_name_prefix,
        match &opts.c {
            Command::Local(_) => "local",
            Command::Subscribe(c) => &c.name,
        }
    );

    let mut rt = toy_rt::RuntimeBuilder::new()
        .thread_name(thread_name)
        .worker_threads(opts.worker)
        .build()
        .unwrap();

    tracing::info!("start supervisor for:{:?}", opts);

    match &opts.c {
        Command::Local(c) => {
            let _guard = initialize_log(&c.log)?;
            let g = get_graph(&c.graph)?;
            let (sv, _, _) = toy::supervisor::local(ExecutorFactory, app);

            rt.block_on(async {
                let _ = sv.oneshot(g).await;
            });
        }
        Command::Subscribe(c) => {
            let _guard = initialize_log(&c.log)?;
            let token = get_credential(&c.user, &c.kid, &c.credential)
                .map_err(|e| Error::read_credential_error(e))?;
            let auth = toy::api_client::auth::Auth::with_bearer_token(&c.user, &token);

            let client = HttpApiClient::new(&c.api_root, auth).unwrap();
            let (sv, _, _) = toy::supervisor::subscribe(&c.name, ExecutorFactory, app, client);

            rt.block_on(async {
                let _ = sv.run().await;
            });
        }
    };

    Ok(())
}

fn initialize_log(opt: &LogOption) -> Result<LogGuard, Error> {
    match opt.log {
        Some(ref path) => match (path.as_path().parent(), path.as_path().file_name()) {
            (Some(dir), Some(prefix)) => {
                toy_tracing::file(dir, prefix, toy_tracing::LogRotation::Never)
                    .map_err(|x| x.into())
            }
            _ => Err(Error::invalid_log_path()),
        },
        None => toy_tracing::console().map_err(|x| x.into()),
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

fn get_graph(file: &PathBuf) -> Result<Graph, Error> {
    let mut f = File::open(file)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let v: Value = match file.as_path().extension() {
        Some(v) => match v.to_str() {
            Some("yml") | Some("yaml") => toy_pack_yaml::unpack(&buffer)?,
            _ => toy_pack_json::unpack(buffer.as_bytes())?,
        },
        None => toy_pack_json::unpack(buffer.as_bytes())?,
    };

    let g = Graph::from(v)?;

    Ok(g)
}

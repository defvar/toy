use clap::{Clap, ValueHint};
use std::path::PathBuf;

#[derive(Clap, Debug)]
pub struct Config {
    #[clap(short, long, default_value = "https://localhost:3030")]
    pub api_root: String,
    #[clap(short, long, env = "TOY_API_CLIENT_USER")]
    pub user: String,
    #[clap(short, long, env = "TOY_API_CLIENT_CREDENTIAL")]
    pub credential: String,
    #[clap(short, long, env = "TOY_API_CLIENT_KID")]
    pub kid: String,
}

#[derive(Clap, Debug)]
pub struct ListCommand {
    pub resource: String,
    #[clap(short, long)]
    pub name: Option<String>,
    #[clap(short, long)]
    pub pretty: Option<bool>,
}

#[derive(Clap, Debug)]
pub struct PutCommand {
    pub resource: String,
    #[clap(short, long)]
    pub name: String,
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub file: PathBuf,
}

#[derive(Clap, Debug)]
pub enum Command {
    List(ListCommand),
    Put(PutCommand),
}

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub c: Command,
    #[clap(flatten)]
    pub config: Config,
}

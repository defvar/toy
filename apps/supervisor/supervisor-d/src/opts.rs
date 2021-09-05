use clap::{Clap, ValueHint};
use std::path::PathBuf;

#[derive(Clap, Debug)]
pub struct LogOption {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub log: Option<PathBuf>,
}

#[derive(Clap, Debug)]
pub struct Subscribe {
    pub name: String,
    #[clap(short, long, default_value = "https://localhost:3030")]
    pub api_root: String,
    #[clap(short, long, env = "TOY_API_CLIENT_USER")]
    pub user: String,
    #[clap(short, long, env = "TOY_API_CLIENT_CREDENTIAL", value_hint = ValueHint::FilePath)]
    pub credential: String,
    #[clap(short, long, env = "TOY_API_CLIENT_KID")]
    pub kid: String,
    #[clap(flatten)]
    pub log: LogOption,
}

#[derive(Clap, Debug)]
pub struct Local {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub graph: PathBuf,
    #[clap(flatten)]
    pub log: LogOption,
}

#[derive(Clap, Debug)]
pub enum Command {
    Local(Local),
    Subscribe(Subscribe),
}

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub c: Command,
    #[clap(short, long, default_value = "4")]
    pub worker: usize,
    #[clap(short, long, default_value = "supervisor")]
    pub thread_name_prefix: String,
}

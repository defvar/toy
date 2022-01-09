use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct LogOption {
    #[clap(short, long, env = "TOY_CTL_LOG_PATH", value_hint = ValueHint::FilePath)]
    pub log: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(
        short,
        long,
        env = "TOY_API_ROOT",
        default_value = "https://localhost:3030"
    )]
    pub api_root: String,
    #[clap(short, long, env = "TOY_API_CLIENT_USER")]
    pub user: String,
    #[clap(short, long, env = "TOY_API_CLIENT_CREDENTIAL")]
    pub credential: String,
    #[clap(short, long, env = "TOY_API_CLIENT_KID")]
    pub kid: String,
    #[clap(flatten)]
    pub log: LogOption,
}

#[derive(Parser, Debug)]
pub struct FindCommand {
    pub resource: String,
    #[clap(short, long)]
    pub name: String,
    #[clap(short, long)]
    pub pretty: Option<bool>,
}

#[derive(Parser, Debug)]
pub struct ListCommand {
    pub resource: String,
    #[clap(short, long)]
    pub pretty: bool,
}

#[derive(Parser, Debug)]
pub struct PutCommand {
    pub resource: String,
    #[clap(short, long)]
    pub name: String,
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub file: PathBuf,
    #[clap(short, long)]
    pub pretty: Option<bool>,
}

#[derive(Parser, Debug)]
pub struct PostCommand {
    pub resource: String,
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub file: PathBuf,
}

#[derive(Parser, Debug)]
pub enum Command {
    Find(FindCommand),
    List(ListCommand),
    Put(PutCommand),
    Post(PostCommand),
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub c: Command,
    #[clap(flatten)]
    pub config: Config,
}

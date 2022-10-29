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
    #[clap(subcommand)]
    pub resource: FindResources,
    #[clap(short, long)]
    pub pretty: bool,
}

#[derive(Parser, Debug)]
pub struct ListCommand {
    #[clap(subcommand)]
    pub resource: ListResources,
    #[clap(short, long)]
    pub pretty: bool,
}

#[derive(Parser, Debug)]
pub struct PutCommand {
    #[clap(subcommand)]
    pub resource: PutResources,
    #[clap(short, long)]
    pub pretty: bool,
}

#[derive(Parser, Debug)]
pub struct PostCommand {
    #[clap(subcommand)]
    pub resource: PostResources,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Find resource by name and key.
    Find(FindCommand),
    /// List resource by name.
    List(ListCommand),
    /// Add or modify resource by json file.
    /// Resource is modified when a key is matched.
    Put(PutCommand),
    /// Add resource by json file.
    Post(PostCommand),
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub c: Command,
    #[clap(flatten)]
    pub config: Config,
}

/////////////////
// resources
/////////////////

#[derive(Debug, Parser)]
pub enum FindResources {
    Supervisors(FindResourceCommand),
    Services(FindResourceCommand),
    Graphs(FindResourceCommand),
}

#[derive(Debug, Parser)]
pub struct FindResourceCommand {
    #[clap(short, long)]
    pub name: String,
}

#[derive(Debug, Parser)]
pub enum ListResources {
    Supervisors(ListResourceCommand),
    Services(ListResourceCommand),
    Roles(ListResourceCommand),
    RoleBindings(ListResourceCommand),
    Tasks(ListResourceCommand),
}

#[derive(Debug, Parser)]
pub struct ListResourceCommand {
    #[clap(short, long)]
    pub opt: Option<String>,
}

#[derive(Debug, Parser)]
pub enum PostResources {
    Tasks(PostResourceCommand),
}

#[derive(Debug, Parser)]
pub struct PostResourceCommand {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub file: PathBuf,
}

#[derive(Debug, Parser)]
pub enum PutResources {
    Roles(PutResourceCommand),
    RoleBindings(PutResourceCommand),
    Graphs(PutResourceCommand),
}

#[derive(Debug, Parser)]
pub struct PutResourceCommand {
    #[clap(short, long)]
    pub name: String,
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub file: PathBuf,
}

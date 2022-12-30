use clap::{Parser, ValueEnum, ValueHint};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Parser, Debug)]
pub struct LogOption {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub log: Option<PathBuf>,
    #[clap(long, env = "TOY_SUPERVISOR_TOKIO_CONSOLE_HOST")]
    pub tokio_console_host: Option<String>,
    #[clap(long, env = "TOY_SUPERVISOR_TOKIO_CONSOLE_PORT")]
    pub tokio_console_port: Option<String>,
    #[clap(long, env = "TOY_LOG_ANSI", default_value = "false")]
    pub ansi: bool,
    #[clap(
        long,
        env = "TOY_LOG_FORMAT",
        default_value = "text",
        value_enum,
        value_parser
    )]
    pub format: LogFormat,
}

#[derive(Parser, Debug)]
pub struct Subscribe {
    #[clap(short, long, env = "TOY_SUPERVISOR_NAME")]
    pub name: String,
    #[clap(
        short = 'H',
        long,
        env = "TOY_SUPERVISOR_API_HOST",
        default_value = "127.0.0.1"
    )]
    pub host: String,
    #[clap(short, long, env = "TOY_SUPERVISOR_API_PORT", default_value = "3031")]
    pub port: String,
    #[clap(
        short,
        long,
        env = "TOY_API_ROOT",
        default_value = "https://localhost:3030"
    )]
    pub api_root: String,
    #[clap(short, long, env = "TOY_API_CLIENT_USER")]
    pub user: String,
    #[clap(short, long, env = "TOY_API_CLIENT_CREDENTIAL", value_hint = ValueHint::FilePath)]
    pub credential: String,
    #[clap(short, long, env = "TOY_API_CLIENT_KID")]
    pub kid: String,
    #[clap(flatten)]
    pub log: LogOption,
    #[clap(
        long,
        env = "TOY_SUPERVISOR_HEART_BEAT_INTERVAL",
        default_value = "10000"
    )]
    pub heart_beat_interval_mills: u64,
    #[clap(
        long,
        env = "TOY_SUPERVISOR_EVENT_EXPORT_INTERVAL",
        default_value = "10000"
    )]
    pub event_export_interval_mills: u64,
}

#[derive(Parser, Debug)]
pub struct Local {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub graph: PathBuf,
    #[clap(flatten)]
    pub log: LogOption,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Local and one shot execution.
    Local(Local),
    /// Works in cooperation with the api server.
    Subscribe(Subscribe),
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub c: Command,
    #[clap(short, long, default_value = "4")]
    pub worker: usize,
    #[clap(short, long, default_value = "supervisor")]
    pub thread_name_prefix: String,
}

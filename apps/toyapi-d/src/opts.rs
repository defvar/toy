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
    #[clap(long, env = "TOY_API_TOKIO_CONSOLE_HOST")]
    pub tokio_console_host: Option<String>,
    #[clap(long, env = "TOY_API_TOKIO_CONSOLE_PORT")]
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
pub struct Opts {
    #[clap(short = 'H', long, env = "TOY_API_HOST", default_value = "127.0.0.1")]
    pub host: String,
    #[clap(short, long, env = "TOY_API_PORT", default_value = "3030")]
    pub port: String,
    #[clap(long, env = "TOY_API_TLS_CERT_PATH", value_hint = ValueHint::FilePath)]
    pub cert_path: String,
    #[clap(long, env = "TOY_API_TLS_KEY_PATH", value_hint = ValueHint::FilePath)]
    pub key_path: String,
    #[clap(long, env = "TOY_API_TLS_PUB_PATH", value_hint = ValueHint::FilePath)]
    pub pub_path: String,
    #[clap(
        long,
        env = "TOY_API_TLS_SECRET_KEY",
        default_value = "__TLS_SECRET_KID__"
    )]
    pub tls_secret_key: String,
    #[clap(flatten)]
    pub log: LogOption,
    #[clap(short, long, default_value = "4")]
    pub worker: usize,
    #[clap(short, long, default_value = "toyapid")]
    pub thread_name_prefix: String,
    #[clap(long, env = "TOY_API_DISPATCH_INTERVAL", default_value = "3000")]
    pub dispatch_interval_mills: u64,
    #[clap(
        long,
        env = "TOY_API_CLEAN_SUPERVISOR_INTERVAL",
        default_value = "10000"
    )]
    pub clean_supervisor_interval_mills: u64,
}

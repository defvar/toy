use clap::Parser;

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
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(flatten)]
    pub config: Config,
}

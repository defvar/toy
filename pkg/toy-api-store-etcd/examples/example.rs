use toy_api_store_etcd::error::StoreEtcdError;
use toy_pack::Unpack;

#[derive(Debug, Unpack)]
struct Test {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), StoreEtcdError> {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    let c = toy_api_store_etcd::client::Client::new("http://localhost:2379");
    let r = c.get("/hoge1/aiueo").await?.json::<Test>()?;
    log::info!("{:?}", r);
    Ok(())
}

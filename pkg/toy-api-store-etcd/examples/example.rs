use toy_api_store_etcd::error::StoreEtcdError;
use toy_pack::{Pack, Unpack};

#[derive(Debug, Pack, Unpack)]
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

    let c = toy_api_store_etcd::Client::new("http://localhost:2379").unwrap();
    let data = Test {
        name: "aiueo".to_string(),
        age: 32,
    };
    let key = "/hoge1/aiueo";
    let data_json = toy_pack_json::pack_to_string(&data).unwrap();
    let put_res = c.create(key, data_json).await?;
    log::info!("{:?}", put_res);
    if !put_res.is_success() {
        log::info!("key:{:?} already exists.", key);
        log::info!("remove!");
        let range_res = c.get(key).await?.json::<Test>()?;
        let remove_res = c.remove(key, range_res.get(0).unwrap().version()).await?;
        log::info!("{:?}", remove_res);
    }
    let range_res = c.get(key).await?.json::<Test>()?;
    log::info!("{:?}", range_res);
    Ok(())
}

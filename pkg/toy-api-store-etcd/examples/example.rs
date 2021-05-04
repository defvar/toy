use futures_util::StreamExt;
use toy_api_server::store::error::StoreError;
use toy_h::impl_reqwest::ReqwestClient;
use toy_pack::{Pack, Unpack};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, Pack, Unpack)]
struct Test {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), StoreError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let prefix = "/hoge1";
    let key = "/hoge1/aiueo";

    let c = ReqwestClient::from(toy_h::reqwest::Client::builder().build().unwrap());
    let c = toy_api_store_etcd::Client::new(c, "http://localhost:2379").unwrap();
    let data = Test {
        name: "aiueo".to_string(),
        age: 32,
    };
    let data_json = toy_pack_json::pack_to_string(&data).unwrap();

    // create new kv
    let put_res = c.create(key, data_json.clone()).await?;
    tracing::info!("create {:?}", put_res);

    // duplicate key..
    let put_res = c.create(key, data_json.clone()).await?;
    tracing::info!("create(2) {:?}", put_res);

    // update
    let range_res = c.get(key).await?.value()?.unwrap();
    let upd_res = c
        .update(key, data_json.clone(), range_res.version())
        .await?;
    tracing::info!("update {:?}", upd_res);

    // list
    let range_res = c.list(prefix).await?.values()?;
    tracing::info!("list {:?}", range_res);

    // remove
    let range_res = c.get(key).await?.value()?.unwrap();
    let rm_res = c.remove(key, range_res.version()).await?;
    tracing::info!("remove {:?}", rm_res);

    // watch...
    let mut watch_res = c.watch("/hoge1").await?;
    while let Some(item) = watch_res.next().await {
        tracing::info!("watch {:?}", item);
    }

    Ok(())
}

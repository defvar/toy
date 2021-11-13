use toy_glogging::error::GLoggingError;
use toy_glogging::models::{ListRequest, TailRequest};
use toy_glogging::Client;
use toy_h::impl_reqwest::ReqwestClient;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), GLoggingError> {
    dotenv::dotenv().ok();

    let _builder = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let c = ReqwestClient::from(toy_h::reqwest::Client::builder().build().unwrap());

    let token =
        toy_glogging::auth::request_token(c.clone(), toy_glogging::auth::Scope::LoggingRead)
            .await?;

    let resouce = std::env::var("TOY_EXAMPLE_GLOGGING_LOG_NAME").expect("env not found");

    let c = Client::from(c);

    //
    // list
    //
    let req = ListRequest::from_resource_name(&resouce)
        .with_filter("(labels.operation = \"last\" OR labels.operation = \"first\") AND timestamp >= \"2021-11-01\"");
    let r = c.list(&token, req).await;
    println!("{:?}", r);

    Ok(())
}

use toy_tail::handlers::PrintHandler;
use toy_tail::{watch, Handler, RegexParser, TailContext};
use toy_tail_glogging::GLoggingHandler;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let path = "/tmp/toy";
    let prefix = "hello.example.log";

    let log_name = std::env::var("TOY_GLOGGING_LOG_NAME")
        .expect("not found log name. please set env TOY_GLOGGING_LOG_NAME");

    tracing::info!("watching dir:{}, prefix:{}", path, prefix);
    let parser = RegexParser::new();
    if let Err(e) = parser {
        tracing::error!("regex build error. {}", e);
        return;
    }

    // runtime for tail handler
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(3)
        .thread_name("tail-worker")
        .enable_all()
        .build()
        .unwrap();

    let c = toy_h::impl_reqwest::ReqwestClient::new().unwrap();
    let handlers: Vec<Box<dyn Handler>> = vec![
        Box::new(GLoggingHandler::from(c, log_name, 100)),
        Box::new(PrintHandler::new()),
    ];

    let (mut ctx, mut timer) = TailContext::new(handlers, parser.unwrap());
    rt.spawn(async move { timer.run().await });

    let (tx, rx) = std::sync::mpsc::channel();

    let tx = tx.clone();
    rt.spawn(async move {
        match watch(path, prefix, &mut ctx).await {
            Ok(_) => {
                tracing::info!("watch end.");
            }
            Err(e) => {
                tracing::error!("error: {:?}", e);
            }
        }
        let _ = tx.send(());
        drop(tx);
        std::future::ready(()).await
    });

    let _ = rx.recv();
}

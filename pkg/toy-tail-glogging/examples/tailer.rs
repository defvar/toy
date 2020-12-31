use toy_tail::{watch, RegexParser, TailContext};
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

    println!("watching dir:{}, prefix:{}", path, prefix);
    let parser = RegexParser::new();
    if let Err(e) = parser {
        println!("regex build error. {}", e);
        return;
    }

    // runtime for tail handler
    let rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(3)
        .thread_name("tail-worker")
        .enable_all()
        .build()
        .unwrap();

    let c = toy_glogging::reqwest::Client::builder().build().unwrap();

    let (mut ctx, mut timer) = TailContext::new(GLoggingHandler::from(c, 10), parser.unwrap());
    rt.spawn(async move { timer.run().await });

    let (tx, rx) = std::sync::mpsc::channel();

    let tx = tx.clone();
    rt.spawn(async move {
        match watch(path, prefix, &mut ctx).await {
            Ok(_) => {
                println!("watch end.");
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
        let _ = tx.send(());
        drop(tx);
        std::future::ready(()).await
    });

    let _ = rx.recv();
}

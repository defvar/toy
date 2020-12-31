use toy_tail::{watch, PrintHandler, RegexParser, TailContext};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let path = "/tmp/toy";
    let prefix = "hello.example.log";

    // runtime for tail handler
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .thread_name("tail-worker")
        .core_threads(4)
        .enable_all()
        .build()
        .unwrap();

    println!("watching dir:{}, prefix:{}", path, prefix);
    let parser = RegexParser::new();
    if let Err(e) = parser {
        println!("regex build error. {}", e);
        return;
    }

    let (mut ctx, mut timer) = TailContext::new(PrintHandler::new(), parser.unwrap());
    rt.spawn(async move { timer.run().await });

    let (tx, rx) = std::sync::mpsc::channel();
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

use toy_tail::handlers::PrintHandler;
use toy_tail::{watch, Handler, RegexParser, TailContext};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let path = "/private/tmp/toy";
    let prefix = "hello.example.log";

    // runtime for tail handler
    let rt = toy_rt::RuntimeBuilder::new()
        .thread_name("toy-tail")
        .worker_threads(4)
        .build()
        .unwrap();

    tracing::info!("watching dir:{}, prefix:{}", path, prefix);
    let regex = r"(?x)
        (?P<datetime>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d*[+-]\d{2}:\d{2})\s+
        (?P<level>\S+)\s+
        (?P<thread_name>\S+)\s+
        (?P<thread_id>\S+)\s+
        Task\{\s*task=(?P<task_id>\S+)\s*(graph=(?P<graph>\S+))?\s*(uri=(?P<uri>\S+))?\s*}:\s+
        (?P<target>\S+):\s+
        (?P<time>close\s*time\.busy=(?P<busy>\S+)\s*time\.idle=(?P<idle>\S+)?)?\s?
        (?P<message>.*)?
    ";
    let parser = RegexParser::new(regex);
    if let Err(e) = parser {
        tracing::error!("regex build error. {}", e);
        return;
    }

    let handlers: Vec<Box<dyn Handler>> = vec![Box::new(PrintHandler::new())];

    let (mut ctx, mut timer) = TailContext::new(handlers, parser.unwrap());
    rt.spawn(async move { timer.run().await });

    let (tx, rx) = std::sync::mpsc::channel();
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

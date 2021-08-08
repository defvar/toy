use std::time::Duration;
use tokio::io::AsyncWriteExt;
use toy_tail::handlers::PrintHandler;
use toy_tail::{Handler, RegexParser};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // runtime for tail handler
    let rt = toy_rt::RuntimeBuilder::new()
        .thread_name("toy-tail")
        .worker_threads(4)
        .build()
        .unwrap();

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

    let (ctx, mut timer) = toy_tail::listeners::TcpContext::new(handlers, parser.unwrap());
    rt.spawn(async move { timer.run().await });

    let (tx, rx) = std::sync::mpsc::channel();
    rt.spawn(async move {
        match ctx.listen().await {
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

    std::thread::sleep(Duration::from_secs(1));

    rt.spawn(async move {
        let mut st = tokio::net::TcpStream::connect("127.0.0.1:6060")
            .await
            .unwrap();
        let _ = st.write(b"2021-08-07T11:54:04.565605+00:00  INFO toy-worker ThreadId(06) Task{task=123 graph=example-tick uri=awaiter}: toy_executor::executor: all upstream finish. awaiter.").await;
    });

    let _ = rx.recv();
}

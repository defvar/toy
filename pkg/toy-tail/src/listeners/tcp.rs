use crate::flush_timer::FlushTimer;
use crate::handler::Handlers;
use crate::parsers::Parser;
use crate::{Handler, TailConfig};
use crate::{Line, TailError};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub struct TcpContext<T> {
    parser: T,
    handlers: Handlers,
    buffer_capacity: usize,
    addrs: Vec<SocketAddr>,
}

impl<T> TcpContext<T>
where
    T: Parser + Send + Sync + 'static,
{
    pub fn new(handlers: Vec<Box<dyn Handler>>, parser: T) -> (Self, FlushTimer) {
        TcpContext::with_capacity(handlers, parser, TailConfig::default())
    }

    pub fn with_capacity(
        handlers: Vec<Box<dyn Handler>>,
        parser: T,
        config: TailConfig,
    ) -> (Self, FlushTimer) {
        let (hs, ft) = Handlers::new(handlers, &config);
        (
            Self {
                handlers: hs,
                parser,
                buffer_capacity: config.buffer_capacity(),
                addrs: config.addrs().to_vec(),
            },
            ft,
        )
    }

    pub async fn listen(self) -> Result<(), TailError> {
        let listener = TcpListener::bind(self.addrs.as_slice()).await?;
        let cap = self.buffer_capacity;
        let handlers = Arc::new(self.handlers);
        let parser = Arc::new(self.parser);

        loop {
            let (stream, _) = listener.accept().await?;
            let handlers = Arc::clone(&handlers);
            let parser = Arc::clone(&parser);
            toy_rt::spawn(async move {
                tracing::info!("spawn");
                let mut flamed = Framed::new(stream, LinesCodec::new());
                let mut line_buffer = Line::with_capacity(cap, 1);

                while let Some(frame) = flamed.next().await {
                    match frame {
                        Ok(line) => {
                            line_buffer.push(line.as_bytes());
                            let fl = parser.parse(&line_buffer);
                            if let Err(e) = handlers.handle(fl, &line_buffer).await {
                                tracing::error!("error:{:?}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("error:{:?}", e);
                        }
                    }
                    line_buffer.clear();
                }
            });
        }
    }
}

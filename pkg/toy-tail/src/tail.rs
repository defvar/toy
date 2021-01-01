use crate::{Handler, LineReader, RegexParser, TailConfig, TailError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use toy_text_parser::dfa::ByteParserBuilder;
use toy_text_parser::Line;

pub struct TailContext {
    position: u64,
    line_buffer: Line,
    reader: HashMap<PathBuf, LineReader<File>>,
    parser: RegexParser,
    last_handle_at: Arc<Mutex<Option<Instant>>>,
    handlers: Arc<Mutex<Vec<Box<dyn Handler>>>>,
}

pub struct FlushTimer {
    last_handle_at: Arc<Mutex<Option<Instant>>>,
    prev_handle_at: Option<Instant>,
    check_interval_millis: u64,
    threshold_millis: u64,
    handlers: Arc<Mutex<Vec<Box<dyn Handler>>>>,
}

impl TailContext {
    pub fn new(handler: Vec<Box<dyn Handler>>, parser: RegexParser) -> (Self, FlushTimer) {
        TailContext::with_capacity(handler, parser, TailConfig::default())
    }

    pub fn with_capacity(
        handler: Vec<Box<dyn Handler>>,
        parser: RegexParser,
        config: TailConfig,
    ) -> (Self, FlushTimer) {
        let handler = Arc::new(Mutex::new(handler));
        let last_handle_at = Arc::new(Mutex::new(None));
        (
            Self {
                position: 0,
                line_buffer: Line::with_capacity(config.buffer_capacity(), 1),
                reader: HashMap::new(),
                parser,
                handlers: Arc::clone(&handler),
                last_handle_at: Arc::clone(&last_handle_at),
            },
            FlushTimer {
                handlers: Arc::clone(&handler),
                last_handle_at: Arc::clone(&last_handle_at),
                prev_handle_at: None,
                check_interval_millis: config.check_interval_millis(),
                threshold_millis: config.threshold_millis(),
            },
        )
    }

    pub fn is_reading<P: AsRef<Path>>(&self, path: P) -> bool {
        self.reader.contains_key(path.as_ref())
    }

    pub fn remove<P: AsRef<Path>>(&mut self, path: P) {
        let _ = self.reader.remove(path.as_ref());
    }

    pub fn follow<P: AsRef<Path>>(&mut self, path: P, tail: bool) -> Result<(), TailError> {
        let mut file = File::open(&path)?;
        let seek = if tail {
            SeekFrom::End(0)
        } else {
            SeekFrom::Start(self.position)
        };
        self.position = file.seek(seek)?;
        match self.reader.get_mut(path.as_ref()) {
            Some(reader) => {
                reader.next(BufReader::new(file));
            }
            None => {
                let p = ByteParserBuilder::default().build();
                self.reader.insert(
                    path.as_ref().to_path_buf(),
                    LineReader::new(p, BufReader::new(file)),
                );
            }
        }

        Ok(())
    }

    pub async fn read<P: AsRef<Path>>(&mut self, path: P) -> Result<(), TailError> {
        match self.reader.get_mut(path.as_ref()) {
            Some(reader) => {
                while reader.read(&mut self.line_buffer)? {
                    let size = self.line_buffer.len_bytes();
                    if size > 0 {
                        self.position += size as u64;
                        let fl = self.parser.parse(&self.line_buffer);
                        {
                            let mut handle_at = self.last_handle_at.lock().await;
                            let mut handlers = self.handlers.lock().await;
                            let now = Instant::now();
                            let parsed = fl.is_some();
                            if handlers.len() == 1 {
                                let h = handlers.get_mut(0).unwrap();
                                h.flagments(fl).await?;
                                h.raw(&self.line_buffer, parsed).await?;
                            } else {
                                for h in handlers.iter_mut() {
                                    h.flagments(fl.clone()).await?;
                                    h.raw(&self.line_buffer, parsed).await?;
                                }
                            }

                            *handle_at = Some(now);
                        }
                        self.line_buffer.clear();
                    }
                }
            }
            None => (),
        }

        Ok(())
    }

    pub async fn handler_names(&self) -> Vec<String> {
        let mut buf = Vec::new();
        {
            let handlers = self.handlers.lock().await;
            for h in handlers.iter() {
                buf.push(h.name().to_string());
            }
        }
        buf
    }
}

impl FlushTimer {
    pub async fn run(&mut self) {
        let mut interval = tokio::time::interval(Duration::from_millis(self.check_interval_millis));

        loop {
            interval.tick().await;

            let now = Instant::now();

            let handle_at = self.last_handle_at.lock().await;
            if self.prev_handle_at == *handle_at {
                continue;
            }

            match *handle_at {
                Some(last)
                    if now.duration_since(last) > Duration::from_millis(self.threshold_millis) =>
                {
                    let mut handler = self.handlers.lock().await;
                    self.prev_handle_at = Some(last);
                    for h in handler.iter_mut() {
                        let r = h.flush().await;
                        if let Err(e) = r {
                            tracing::error!("error flush timer. error:{:?}", e);
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

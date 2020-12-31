use crate::{Handler, LineReader, RegexParser, TailError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use toy_text_parser::dfa::ByteParserBuilder;
use toy_text_parser::Line;

pub struct TailContext<T> {
    position: u64,
    line_buffer: Line,
    reader: HashMap<PathBuf, LineReader<File>>,
    parser: RegexParser,
    handler: Arc<Mutex<T>>,
    last_handle_at: Arc<Mutex<Option<Instant>>>,
}

pub struct FlushTimer<T> {
    handler: Arc<Mutex<T>>,
    last_handle_at: Arc<Mutex<Option<Instant>>>,
    prev_handle_at: Option<Instant>,
    check_interval_secs: u64,
    threshold_secs: u64,
}

const DEFAULT_CAPACITY: usize = 8 * (1 << 10);

impl<T> TailContext<T>
where
    T: Handler,
{
    pub fn new(handler: T, parser: RegexParser) -> (Self, FlushTimer<T>) {
        TailContext::with_capacity(handler, parser, DEFAULT_CAPACITY)
    }

    pub fn with_capacity(
        handler: T,
        parser: RegexParser,
        capacity: usize,
    ) -> (Self, FlushTimer<T>) {
        let handler = Arc::new(Mutex::new(handler));
        let last_handle_at = Arc::new(Mutex::new(None));
        (
            Self {
                position: 0,
                line_buffer: Line::with_capacity(capacity, 1),
                reader: HashMap::new(),
                parser,
                handler: Arc::clone(&handler),
                last_handle_at: Arc::clone(&last_handle_at),
            },
            FlushTimer {
                handler: Arc::clone(&handler),
                last_handle_at: Arc::clone(&last_handle_at),
                prev_handle_at: None,
                check_interval_secs: 2,
                threshold_secs: 2,
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
                            let mut handler = self.handler.lock().await;
                            let now = Instant::now();
                            handler.flagments(fl).await?;
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
}

impl<T> FlushTimer<T>
where
    T: Handler,
{
    pub async fn run(&mut self) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.check_interval_secs));

        loop {
            interval.tick().await;

            let now = Instant::now();
            let r = {
                let handle_at = self.last_handle_at.lock().await;
                if self.prev_handle_at == *handle_at {
                    continue;
                }

                match *handle_at {
                    Some(last)
                        if now.duration_since(last) > Duration::from_secs(self.threshold_secs) =>
                    {
                        let mut handler = self.handler.lock().await;
                        self.prev_handle_at = Some(last);
                        handler.flush().await
                    }
                    _ => Ok(()),
                }
            };
            if let Err(e) = r {
                tracing::error!("error flush timer. error:{:?}", e);
            }
        }
    }
}

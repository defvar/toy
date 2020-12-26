use crate::{Handler, LineReader, RegexParser, TailError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use toy_text_parser::dfa::ByteParserBuilder;
use toy_text_parser::Line;

pub struct TailContext<T> {
    position: u64,
    line_buffer: Line,
    reader: HashMap<PathBuf, LineReader<File>>,
    parser: RegexParser,
    handler: T,
}

const DEFAULT_CAPACITY: usize = 8 * (1 << 10);

impl<T> TailContext<T>
where
    T: Handler,
{
    pub fn new(handler: T, parser: RegexParser) -> Self {
        Self {
            position: 0,
            line_buffer: Line::with_capacity(DEFAULT_CAPACITY, 1),
            reader: HashMap::new(),
            parser,
            handler,
        }
    }

    pub fn with_capacity(handler: T, parser: RegexParser, capacity: usize) -> Self {
        Self {
            position: 0,
            line_buffer: Line::with_capacity(capacity, 1),
            reader: HashMap::new(),
            parser,
            handler,
        }
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

    pub fn read<P: AsRef<Path>>(&mut self, path: P) -> Result<(), TailError> {
        match self.reader.get_mut(path.as_ref()) {
            Some(reader) => {
                while reader.read(&mut self.line_buffer)? {
                    let size = self.line_buffer.len_bytes();
                    if size > 0 {
                        self.position += size as u64;
                        let fl = self.parser.parse(&self.line_buffer);
                        self.handler.flagments(fl)?;
                        self.handler.raw(&self.line_buffer)?;
                        self.line_buffer.clear();
                    }
                }
            }
            None => (),
        }

        Ok(())
    }
}

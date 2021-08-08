use crate::{Flagments, Handler, Line, TailError};
use async_trait::async_trait;
use chrono::NaiveDateTime;

/// Sample handler.
/// println.
pub struct PrintHandler {
    buffer: Vec<String>,
}

impl PrintHandler {
    pub fn new() -> PrintHandler {
        PrintHandler { buffer: Vec::new() }
    }
}

#[async_trait]
impl Handler for PrintHandler {
    fn name(&self) -> &'static str {
        "Print"
    }

    async fn flagments(&mut self, fl: Flagments<'_>) -> Result<(), TailError> {
        if fl.is_some() {
            let unix_time = match fl.datetime() {
                Some(datetime) => NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S%.f%z")
                    .map(|x| x.timestamp_nanos())
                    .unwrap_or(0),
                None => 0,
            };
            self.buffer.push(format!("{}, {}", fl, unix_time));
        }
        if self.buffer.len() > 10 {
            self.buffer.iter().for_each(|x| {
                println!("{}", x);
            });
            self.buffer.clear();
        }
        Ok(())
    }

    async fn raw(&mut self, raw: &'_ Line, parse_successed: bool) -> Result<(), TailError> {
        if !parse_successed {
            self.buffer.push(format!("{}", raw));
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), TailError> {
        if self.buffer.len() > 0 {
            self.buffer.iter().for_each(|x| {
                println!("{}", x);
            });
            self.buffer.clear();
        }
        Ok(())
    }
}

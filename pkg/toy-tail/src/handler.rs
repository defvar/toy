use crate::{Flagments, TailError};
use async_trait::async_trait;
use toy_text_parser::Line;

#[async_trait]
pub trait Handler: Send {
    /// handler name for logging.
    fn name(&self) -> &'static str;

    /// Called after parsed one line.
    /// If buffering is required, it must be done on the implementation side.
    async fn flagments(&mut self, fl: Flagments<'_>) -> Result<(), TailError>;

    /// Called after parsed one line.
    async fn raw(&mut self, raw: &'_ Line, parse_successed: bool) -> Result<(), TailError> {
        let _ = raw;
        let _ = parse_successed;
        Ok(())
    }

    /// Called when elapsed threshold by `FlushTimer`.
    async fn flush(&mut self) -> Result<(), TailError>;
}

use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use chrono::{DateTime, Utc};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Range {
    start: Option<DateTime<Utc>>,
    stop: Option<DateTime<Utc>>,
}

impl Range {
    pub fn with(start: Option<DateTime<Utc>>, stop: Option<DateTime<Utc>>) -> Range {
        Self { start, stop }
    }

    pub fn start(v: DateTime<Utc>) -> Range {
        Self {
            start: Some(v),
            stop: None,
        }
    }

    pub fn stop(v: DateTime<Utc>) -> Range {
        Self {
            start: None,
            stop: Some(v),
        }
    }

    pub fn between(start: DateTime<Utc>, stop: DateTime<Utc>) -> Range {
        Self {
            start: Some(start),
            stop: Some(stop),
        }
    }
}

impl FluxPart for Range {
    fn need(&self) -> bool {
        return !(self.start.is_none() && self.stop.is_none());
    }

    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        match (self.start, self.stop) {
            (Some(start), Some(stop)) => writer
                .write(
                    format!(
                        "range(start: {}, stop: {})",
                        start.to_rfc3339(),
                        stop.to_rfc3339()
                    )
                    .as_bytes(),
                )
                .map_err(|e| e.into()),
            (Some(start), None) => writer
                .write(format!("range(start: {})", start.to_rfc3339()).as_bytes())
                .map_err(|e| e.into()),
            (None, Some(stop)) => writer
                .write(format!("range(stop: {})", stop.to_rfc3339()).as_bytes())
                .map_err(|e| e.into()),
            (None, None) => Ok(0),
        }
    }
}

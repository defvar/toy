use crate::{Flagments, TailError};
use chrono::NaiveDateTime;
use toy_text_parser::Line;

pub trait Handler {
    fn flagments(&mut self, fl: Flagments) -> Result<(), TailError>;

    fn raw(&mut self, line: &Line) -> Result<(), TailError>;
}

pub struct PrintHandler;

impl Handler for PrintHandler {
    fn flagments(&mut self, fl: Flagments) -> Result<(), TailError> {
        if fl.is_some() {
            let unix_time = match fl.datetime() {
                Some(datetime) => NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S%.f%z")
                    .map(|x| x.timestamp_nanos())
                    .unwrap_or(0),
                None => 0,
            };
            println!("{} {}", fl, unix_time);
        }
        Ok(())
    }

    fn raw(&mut self, line: &Line) -> Result<(), TailError> {
        println!("{}", line);
        Ok(())
    }
}

pub mod builder;
mod decoder;

use crate::models::flux_table::FluxTable;
use crate::InfluxDBError;
pub use decoder::Decoder;
use std::io::Read;

pub fn decode<R: Read>(reader: R) -> Result<Vec<FluxTable>, InfluxDBError> {
    let d = Decoder::with(reader, 40, 128, 1024);
    d.decode()
}

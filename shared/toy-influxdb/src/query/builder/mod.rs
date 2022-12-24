use crate::InfluxDBError;
use std::io::Write;

mod filter;
mod flux_builder;
mod from;
mod group;
mod limit;
mod pivot;
mod range;
mod rename;
mod sort;

pub use filter::Filter;
pub use flux_builder::{FluxBuilder, Identity, Layer};
pub use from::From;
pub use group::Group;
pub use limit::Limit;
pub use range::Range;
pub use sort::Sort;

pub trait FluxPart {
    fn need(&self) -> bool {
        return true;
    }

    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError>;
}

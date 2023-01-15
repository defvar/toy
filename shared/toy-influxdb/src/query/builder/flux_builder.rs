use crate::query::builder::from::From;
use crate::query::builder::pivot::Pivot;
use crate::query::builder::rename::Rename;
use crate::query::builder::{Drop, Filter, FluxPart, Group, Limit, Range, Sort};
use crate::InfluxDBError;
use chrono::{DateTime, Utc};
use std::fmt;
use std::io::Write;

#[derive(Clone)]
pub struct FluxBuilder<L> {
    parts: L,
}

impl Default for FluxBuilder<Identity> {
    fn default() -> Self {
        FluxBuilder {
            parts: Identity::new(),
        }
    }
}

impl FluxBuilder<Identity> {
    pub fn from(bucket: &str) -> FluxBuilder<Layer<From, Identity>> {
        FluxBuilder {
            parts: Layer::new(From::with(bucket), Identity::new()),
        }
    }
}

impl<L> FluxBuilder<L> {
    pub fn push<T>(self, clause: T) -> FluxBuilder<Layer<T, L>> {
        FluxBuilder {
            parts: Layer::new(clause, self.parts),
        }
    }

    pub fn filter(self, clause: Filter) -> FluxBuilder<Layer<Filter, L>> {
        FluxBuilder {
            parts: Layer::new(clause, self.parts),
        }
    }

    pub fn range(
        self,
        start: Option<DateTime<Utc>>,
        stop: Option<DateTime<Utc>>,
    ) -> FluxBuilder<Layer<Range, L>> {
        FluxBuilder {
            parts: Layer::new(Range::with(start, stop), self.parts),
        }
    }

    pub fn ungroup<'a>(self) -> FluxBuilder<Layer<Group<'a>, L>> {
        FluxBuilder {
            parts: Layer::new(Group::ungroup(), self.parts),
        }
    }

    pub fn group<'a>(self, columns: &[&'a str]) -> FluxBuilder<Layer<Group<'a>, L>> {
        FluxBuilder {
            parts: Layer::new(Group::with(Vec::from(columns)), self.parts),
        }
    }

    pub fn pivot<'a>(
        self,
        row_key: &[&'a str],
        column_key: &[&'a str],
        value_column: &'a str,
    ) -> FluxBuilder<Layer<Pivot<'a>, L>> {
        FluxBuilder {
            parts: Layer::new(
                Pivot::with(Vec::from(row_key), Vec::from(column_key), value_column),
                self.parts,
            ),
        }
    }

    pub fn sort<'a>(self, columns: &[&'a str]) -> FluxBuilder<Layer<Sort<'a>, L>> {
        FluxBuilder {
            parts: Layer::new(Sort::with(Vec::from(columns)), self.parts),
        }
    }

    pub fn rename<'a>(self, map: &[(&'a str, &'a str)]) -> FluxBuilder<Layer<Rename<'a>, L>> {
        FluxBuilder {
            parts: Layer::new(Rename::with(map), self.parts),
        }
    }

    pub fn drop<'a>(self, columns: &[&'a str]) -> FluxBuilder<Layer<Drop<'a>, L>> {
        FluxBuilder {
            parts: Layer::new(Drop::with(Vec::from(columns)), self.parts),
        }
    }

    pub fn limit(self, limit: usize) -> FluxBuilder<Layer<Limit, L>> {
        FluxBuilder {
            parts: Layer::new(Limit::n(limit), self.parts),
        }
    }

    pub fn limit_and_offset(self, limit: usize, offset: usize) -> FluxBuilder<Layer<Limit, L>> {
        FluxBuilder {
            parts: Layer::new(Limit::with(limit, offset), self.parts),
        }
    }
}

impl<L> FluxBuilder<L>
where
    L: FluxPart,
{
    pub fn to_flux(&self) -> Result<String, InfluxDBError> {
        let mut buffer = Vec::new();
        self.parts.to_flux(&mut buffer)?;
        Ok(String::from_utf8(buffer).map_err(|e| InfluxDBError::error(e))?)
    }
}

#[derive(Clone)]
pub struct Layer<Inner, Outer> {
    inner: Inner,
    outer: Outer,
}

impl<Inner, Outer> Layer<Inner, Outer> {
    pub fn new(inner: Inner, outer: Outer) -> Self {
        Self { inner, outer }
    }
}

impl<Inner, Outer> FluxPart for Layer<Inner, Outer>
where
    Inner: FluxPart,
    Outer: FluxPart,
{
    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut size = if self.outer.need() {
            self.outer.to_flux(writer)?
        } else {
            0
        };
        if size > 0 && self.inner.need() {
            size += writer.write(&b" |> "[..])?;
        }
        if self.inner.need() {
            size += self.inner.to_flux(writer)?;
        }

        Ok(size)
    }
}

impl<Inner, Outer> fmt::Debug for Layer<Inner, Outer>
where
    Inner: fmt::Debug,
    Outer: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?},\n{:#?}", self.outer, self.inner)
        } else {
            write!(f, "{:?}, {:?}", self.outer, self.inner)
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Identity {
    _p: (),
}

impl Identity {
    /// Create a new [`Identity`] value
    pub fn new() -> Identity {
        Identity { _p: () }
    }
}

impl FluxPart for Identity {
    fn to_flux<W: Write>(&self, _writer: &mut W) -> Result<usize, InfluxDBError> {
        Ok(0)
    }
}

impl fmt::Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identity").finish()
    }
}

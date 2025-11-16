use crate::models::FieldValue;
use crate::InfluxDBError;
use chrono::{DateTime, Utc};
use std::io::Write;

static DELIMITER_SET: &[u8] = b" ";
static DELIMITER_MEASUREMENT: &[u8] = b",";
static DELIMITER_FIELD_KV: &[u8] = b"=";
static DELIMITER_FIELDS: &[u8] = b",";
static DELIMITER_TAG_KV: &[u8] = b"=";
static DELIMITER_TAGS: &[u8] = b",";

pub trait ToLineProtocol {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError>;
}

pub struct LineProtocolBuilder<'a> {
    records: Vec<LineProtocolRecord<'a>>,
    buf: Option<LineProtocolRecord<'a>>,
}

impl<'a> Default for LineProtocolBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> LineProtocolBuilder<'a> {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            buf: None,
        }
    }

    pub fn start_record(&mut self, measurement: &'a str, timestamp: DateTime<Utc>) -> &mut Self {
        self.buf = Some(LineProtocolRecord::with(
            measurement,
            TagSet::new(),
            FieldSet::new(),
            timestamp,
        ));
        self
    }

    pub fn end_record(&mut self) -> &mut Self {
        let r = self.buf.take().unwrap();
        self.records.push(r);
        self
    }

    pub fn tag(&mut self, k: &'a str, v: &'a str) -> &mut Self {
        let mut r = self
            .buf
            .take()
            .expect("illegal builder state, must be call after start_record.");
        r.tags.push(Tag::with(k, v));
        self.buf = Some(r);
        self
    }

    pub fn field(&mut self, k: &'a str, v: FieldValue) -> &mut Self {
        let mut r = self
            .buf
            .take()
            .expect("illegal builder state, must be call after start_record.");
        r.fields.push(Field::with(k, v));
        self.buf = Some(r);
        self
    }

    pub fn build(self) -> Vec<LineProtocolRecord<'a>> {
        self.records
    }
}

#[derive(Clone, Debug)]
pub struct LineProtocolRecord<'a> {
    measurement: &'a str,
    tags: TagSet<'a>,
    fields: FieldSet<'a>,
    timestamp: DateTime<Utc>,
}

impl<'a> LineProtocolRecord<'a> {
    pub fn with(
        measurement: &'a str,
        tags: TagSet<'a>,
        fields: FieldSet<'a>,
        timestamp: DateTime<Utc>,
    ) -> LineProtocolRecord<'a> {
        Self {
            measurement,
            tags,
            fields,
            timestamp,
        }
    }

    pub fn measurement(&self) -> &'a str {
        self.measurement
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}

impl<'a> ToLineProtocol for LineProtocolRecord<'a> {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        let mut size = 0usize;
        size += writer.write(self.measurement.as_bytes())?;
        size += writer.write(DELIMITER_MEASUREMENT)?;
        size += self.tags.to_lp(writer)?;
        size += writer.write(DELIMITER_SET)?;
        size += self.fields.to_lp(writer)?;
        size += writer.write(DELIMITER_SET)?;

        let mut buf = itoa::Buffer::new();
        let bytes = buf
            .format(self.timestamp.timestamp_nanos_opt().unwrap())
            .as_bytes();
        size += writer.write(bytes)?;
        Ok(size)
    }
}

#[derive(Clone, Debug)]
pub struct TagSet<'a> {
    items: Vec<Tag<'a>>,
}

impl<'a> Default for TagSet<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TagSet<'a> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with<I>(items: I) -> Self
    where
        I: Iterator<Item = Tag<'a>>,
    {
        Self {
            items: items.collect(),
        }
    }

    pub fn push(&mut self, tag: Tag<'a>) {
        self.items.push(tag)
    }
}

impl<'a> ToLineProtocol for TagSet<'a> {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        self.items.iter().try_fold(0usize, |mut acc, x| {
            if acc != 0 {
                acc += writer.write(DELIMITER_TAGS)?;
            }
            acc += x.to_lp(writer)?;
            Ok(acc)
        })
    }
}

#[derive(Clone, Debug)]
pub struct Tag<'a> {
    k: &'a [u8],
    v: &'a [u8],
}

impl<'a> Tag<'a> {
    pub fn with(k: &'a str, v: &'a str) -> Self {
        Self {
            k: k.as_bytes(),
            v: v.as_bytes(),
        }
    }
}

impl<'a> ToLineProtocol for Tag<'a> {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        writer.write_all(self.k)?;
        writer.write_all(DELIMITER_TAG_KV)?;
        writer.write_all(self.v)?;
        Ok(self.k.len() + 1 + self.v.len())
    }
}

#[derive(Clone, Debug)]
pub struct FieldSet<'a> {
    items: Vec<Field<'a>>,
}

impl<'a> Default for FieldSet<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> FieldSet<'a> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with<I>(fields: I) -> FieldSet<'a>
    where
        I: Iterator<Item = Field<'a>>,
    {
        FieldSet {
            items: fields.collect(),
        }
    }

    pub fn push(&mut self, field: Field<'a>) {
        self.items.push(field);
    }
}

impl<'a> ToLineProtocol for FieldSet<'a> {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        self.items.iter().try_fold(0usize, |mut acc, x| {
            if acc != 0 {
                acc += writer.write(DELIMITER_FIELDS)?;
            }
            acc += x.to_lp(writer)?;
            Ok(acc)
        })
    }
}

#[derive(Clone, Debug)]
pub struct Field<'a> {
    k: &'a [u8],
    v: FieldValue,
}

impl<'a> Field<'a> {
    pub fn with(k: &'_ str, v: FieldValue) -> Field<'_> {
        Field { k: k.as_bytes(), v }
    }
}

impl<'a> ToLineProtocol for Field<'a> {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        writer.write_all(self.k)?;
        writer.write_all(DELIMITER_FIELD_KV)?;
        let vlen = self.v.to_lp(writer)?;
        Ok(self.k.len() + 1 + vlen)
    }
}

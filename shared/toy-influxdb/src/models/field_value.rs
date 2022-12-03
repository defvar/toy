use crate::models::line_protocol::ToLineProtocol;
use crate::InfluxDBError;
use chrono::{DateTime, Utc};
use std::io::Write;

#[derive(Clone, Debug)]
pub enum FieldValue {
    Float(f64),
    Integer(i64),
    UInteger(u64),
    String(String),
    Boolean(bool),
    Timestamp(DateTime<Utc>),
}

impl FieldValue {
    pub fn from(field: &str, data_type: &str, value: &str) -> Result<Self, InfluxDBError> {
        match data_type {
            "boolean" => {
                let b = if value.is_empty() {
                    None
                } else {
                    Some(value.as_bytes()[0])
                };
                match b {
                    Some(b'f') | Some(b'F') => Ok(FieldValue::Boolean(false)),
                    Some(b't') | Some(b'T') => Ok(FieldValue::Boolean(true)),
                    _ => Err(InfluxDBError::invalid_field_value(field, "bool", value)),
                }
            }
            "string" => Ok(FieldValue::String(value.to_string())),
            "double" => match value {
                "+Inf" => Ok(FieldValue::Float(f64::INFINITY)),
                "-Inf" => Ok(FieldValue::Float(f64::NEG_INFINITY)),
                _ => value
                    .parse::<f64>()
                    .map_err(|_| InfluxDBError::invalid_field_value(field, "double(f64)", value))
                    .map(|x| FieldValue::Float(x)),
            },
            "unsignedLong" => value
                .parse::<u64>()
                .map_err(|_| InfluxDBError::invalid_field_value(field, "unsignedLong(u64)", value))
                .map(|x| FieldValue::UInteger(x)),
            "long" => value
                .parse::<i64>()
                .map_err(|_| InfluxDBError::invalid_field_value(field, "long(i64)", value))
                .map(|x| FieldValue::Integer(x)),
            "dateTime:RFC3339" | "dateTime:RFC3339Nano" => DateTime::parse_from_rfc3339(value)
                .map_err(|_| InfluxDBError::invalid_field_value(field, data_type, value))
                .map(|x| FieldValue::Timestamp(x.with_timezone(&Utc))),
            _ => Err(InfluxDBError::invalid_field_value(field, data_type, value)),
        }
    }
}

impl ToLineProtocol for FieldValue {
    fn to_lp<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        match self {
            FieldValue::Float(v) => {
                let mut buf = ryu::Buffer::new();
                let bytes = buf.format(*v).as_bytes();
                writer.write(bytes)?;
                Ok(bytes.len())
            }
            FieldValue::Integer(v) => {
                let mut buf = itoa::Buffer::new();
                let bytes = buf.format(*v).as_bytes();
                writer.write(bytes)?;
                Ok(bytes.len())
            }
            FieldValue::UInteger(v) => {
                let mut buf = itoa::Buffer::new();
                let bytes = buf.format(*v).as_bytes();
                writer.write(bytes)?;
                Ok(bytes.len())
            }
            FieldValue::String(v) => {
                writer.write(v.as_bytes())?;
                Ok(v.as_bytes().len())
            }
            FieldValue::Boolean(v) => {
                let bytes = if *v { &b"true"[..] } else { &b"false"[..] };
                writer.write(&bytes)?;
                Ok(bytes.len())
            }
            FieldValue::Timestamp(v) => {
                let mut buf = itoa::Buffer::new();
                let bytes = buf.format(v.timestamp_nanos()).as_bytes();
                writer.write(bytes)?;
                Ok(bytes.len())
            }
        }
    }
}

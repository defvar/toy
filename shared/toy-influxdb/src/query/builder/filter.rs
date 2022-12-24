use crate::models::FieldValue;
use crate::query::builder::FluxPart;
use crate::InfluxDBError;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Filter<'a> {
    inner: Inner<'a>,
}

#[derive(Debug, Clone)]
enum Inner<'a> {
    None,
    Some {
        field: &'a str,
        op: &'a str,
        v: FieldValue,
    },
}

impl<'a> Filter<'a> {
    pub fn none() -> Filter<'a> {
        Self { inner: Inner::None }
    }

    pub fn eq(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: "==", v },
        }
    }

    pub fn ne(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: "!=", v },
        }
    }

    pub fn regex_match(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: "=~", v },
        }
    }

    pub fn regex_not_match(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: "!~", v },
        }
    }

    pub fn less_than(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: "<", v },
        }
    }

    pub fn less_than_or_equal(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: "<=", v },
        }
    }

    pub fn greater_than(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: ">", v },
        }
    }

    pub fn greater_than_or_equal(field: &'a str, v: FieldValue) -> Filter<'a> {
        Self {
            inner: Inner::Some { field, op: ">=", v },
        }
    }
}

impl<'a> FluxPart for Filter<'a> {
    fn need(&self) -> bool {
        return matches!(self.inner, Inner::Some { .. });
    }

    fn to_flux<W: Write>(&self, writer: &mut W) -> Result<usize, InfluxDBError> {
        match &self.inner {
            Inner::None => Ok(0),
            Inner::Some { field, op, v } => {
                let mut r = 0;
                r += writer.write(format!("filter(fn: (r) => r.{} {} ", field, op).as_bytes())?;

                r += writer.write(quote(op, v).as_bytes())?;
                r += v.write(writer, false, true)?;
                r += writer.write(quote(op, v).as_bytes())?;

                r += writer.write(&b")"[..])?;

                Ok(r)
            }
        }
    }
}

fn quote<'a>(op: &'a str, v: &FieldValue) -> &'a str {
    if op == "!~" || op == "=~" {
        "/"
    } else if v.is_string() {
        "\""
    } else {
        ""
    }
}

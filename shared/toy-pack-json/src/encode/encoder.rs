use super::Result;
use crate::EncodeError;
use std::io;

pub struct Encoder<W> {
    pub writer: W,
    pretty: bool,
    current_indent: usize,
    has_value: bool,
}

static INDENT: &'static [u8] = b"  ";

impl<W> Encoder<W> {
    pub fn new(writer: W) -> Encoder<W> {
        Encoder {
            writer,
            pretty: false,
            current_indent: 0,
            has_value: false,
        }
    }

    pub fn pretty(writer: W) -> Encoder<W> {
        Encoder {
            writer,
            pretty: true,
            current_indent: 0,
            has_value: false,
        }
    }
}

impl<W> Encoder<W>
where
    W: io::Write,
{
    #[inline]
    pub fn write_null(&mut self) -> Result<()> {
        self.writer.write_all(b"null").map_err(Into::into)
    }

    #[inline]
    pub fn write_bool(&mut self, v: bool) -> Result<()> {
        let s = if v {
            b"true" as &[u8]
        } else {
            b"false" as &[u8]
        };
        self.writer.write_all(s).map_err(Into::into)
    }

    #[inline]
    pub fn write_u8(&mut self, v: u8) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_u16(&mut self, v: u16) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_u32(&mut self, v: u32) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_u64(&mut self, v: u64) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_i8(&mut self, v: i8) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_i16(&mut self, v: i16) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_i32(&mut self, v: i32) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_i64(&mut self, v: i64) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_f32(&mut self, v: f32) -> Result<()> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_f64(&mut self, v: f64) -> Result<()> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(v);
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    pub fn write_string(&mut self, s: &str) -> Result<()> {
        self.write_string_escaped(s)
    }

    #[inline]
    fn write_string_no_escape(&mut self, s: &str) -> Result<()> {
        self.writer.write_all(s.as_bytes()).map_err(Into::into)
    }

    #[inline]
    fn write_string_escaped(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        let mut start = 0;

        for (i, &byte) in bytes.iter().enumerate() {
            let escape = ESCAPE[byte as usize];
            if escape == 0 {
                continue;
            }

            if start < i {
                self.write_string_no_escape(&s[start..i])?;
            }

            self.write_escape_char(escape)?;

            start = i + 1;
        }

        if start != bytes.len() {
            self.write_string_no_escape(&s[start..])?;
        }

        Ok(())
    }

    #[inline]
    fn write_escape_char(&mut self, c: u8) -> Result<()> {
        let s = match c {
            BB => b"\\b",
            TT => b"\\t",
            NN => b"\\n",
            FF => b"\\f",
            RR => b"\\r",
            QU => b"\\\"",
            BS => b"\\\\",
            c => {
                return Err(EncodeError::error(format!(
                    "not support escape char {:?}",
                    c
                )))
            }
        };
        self.writer.write_all(s).map_err(Into::into)
    }

    #[inline]
    pub fn write_begin_string(&mut self) -> Result<()> {
        self.writer.write_all(b"\"").map_err(Into::into)
    }

    #[inline]
    pub fn write_end_string(&mut self) -> Result<()> {
        self.writer.write_all(b"\"").map_err(Into::into)
    }

    #[inline]
    pub fn write_begin_array(&mut self) -> Result<()> {
        if self.pretty {
            self.current_indent += 1;
            self.has_value = false;
        }
        self.writer.write_all(b"[").map_err(Into::into)
    }

    #[inline]
    pub fn write_end_array(&mut self) -> Result<()> {
        if self.pretty {
            self.current_indent -= 1;
            if self.has_value {
                self.writer.write_all(b"\n")?;
                indent(&mut self.writer, self.current_indent, INDENT)?;
            }
        }
        self.writer.write_all(b"]").map_err(Into::into)
    }

    #[inline]
    pub fn write_begin_array_element(&mut self, first: bool) -> Result<()> {
        let r = if first {
            Ok(())
        } else {
            self.writer.write_all(b",").map_err(Into::into)
        };
        if self.pretty {
            self.writer.write_all(b"\n")?;
            indent(&mut self.writer, self.current_indent, INDENT)?;
        }
        r
    }

    #[inline]
    pub fn write_end_array_element(&mut self) -> Result<()> {
        if self.pretty {
            self.has_value = true;
        }
        Ok(())
    }

    #[inline]
    pub fn write_begin_object(&mut self) -> Result<()> {
        if self.pretty {
            self.current_indent += 1;
            self.has_value = false;
        }
        self.writer.write_all(b"{").map_err(Into::into)
    }

    #[inline]
    pub fn write_end_object(&mut self) -> Result<()> {
        if self.pretty {
            self.current_indent -= 1;
            if self.has_value {
                self.writer.write_all(b"\n")?;
                indent(&mut self.writer, self.current_indent, INDENT)?;
            }
        }
        self.writer.write_all(b"}").map_err(Into::into)
    }

    #[inline]
    pub fn write_begin_object_key(&mut self, first: bool) -> Result<()> {
        let r = if first {
            Ok(())
        } else {
            self.writer.write_all(b",").map_err(Into::into)
        };
        if self.pretty {
            self.writer.write_all(b"\n")?;
            indent(&mut self.writer, self.current_indent, INDENT)?;
        }
        r
    }

    #[inline]
    pub fn write_end_object_key(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    pub fn write_begin_object_value(&mut self) -> Result<()> {
        self.writer.write_all(b":").map_err(Into::into)
    }

    #[inline]
    pub fn write_end_object_value(&mut self) -> Result<()> {
        if self.pretty {
            self.has_value = true;
        }
        Ok(())
    }
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        wr.write_all(s)?;
    }

    Ok(())
}

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const UU: u8 = b'u'; // \x00...\x1F except the ones above
const __: u8 = 0;

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU, // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];

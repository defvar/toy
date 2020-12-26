use crate::data::map::Map;
use core::time::Duration;
use std::fmt;
use std::str::FromStr;
use toy_pack::deser::from_primitive::FromPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    String(String),
    Bytes(Vec<u8>),

    None,
    Some(Box<Value>),

    Seq(Vec<Value>),
    Map(Map<String, Value>),

    TimeStamp(Duration),

    Unit,
}

impl Value {
    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    pub fn as_map(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Map(ref map) => Some(map),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Value::U8(_)
            | Value::U16(_)
            | Value::U32(_)
            | Value::U64(_)
            | Value::I8(_)
            | Value::I16(_)
            | Value::I32(_)
            | Value::I64(_)
            | Value::F32(_)
            | Value::F64(_) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match *self {
            Value::U8(_)
            | Value::U16(_)
            | Value::U32(_)
            | Value::U64(_)
            | Value::I8(_)
            | Value::I16(_)
            | Value::I32(_)
            | Value::I64(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn is_u32(&self) -> bool {
        self.as_u32().is_some()
    }

    pub fn as_u32(&self) -> Option<u32> {
        match *self {
            Value::U32(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_u64(&self) -> bool {
        self.as_u64().is_some()
    }

    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::U64(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_timestamp(&self) -> bool {
        self.as_timestamp().is_some()
    }

    pub fn as_timestamp(&self) -> Option<Duration> {
        match self {
            Value::TimeStamp(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Looks up a value by path.
    /// path is a Unicode string with the reference tokens separated by `.`
    ///
    /// # Example
    ///
    /// ```
    /// # use toy_core::data::Value;
    /// # use toy_core::{map_value, seq_value};
    ///
    /// let v = map_value! {
    ///     "a" => 1,
    ///     "b" => map_value! {
    ///         "x" => 2,
    ///         "y" => seq_value![100, 200],
    ///     }
    /// };
    ///
    /// assert_eq!(v.path("a").unwrap(), &Value::from(1));
    /// assert_eq!(v.path("b.x").unwrap(), &Value::from(2));
    /// assert_eq!(v.path("b.y.1").unwrap(), &Value::from(200));
    /// ```
    pub fn path(&self, path: &str) -> Option<&Value> {
        if path == "" {
            return Some(self);
        }
        let tokens = path.split('.');
        let mut target = self;

        for token in tokens {
            let target_opt = match *target {
                Value::Map(ref map) => map.get(token),
                Value::Seq(ref list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }

    /// Insert a key value pair.
    /// path is a Unicode string with the reference tokens separated by `.`.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Example
    ///
    /// ```
    /// # use toy_core::data::Value;
    /// # use toy_core::map_value;
    ///
    /// let mut v = map_value! {
    ///     "a" => 1,
    /// };
    ///
    /// let expected = map_value! {
    ///     "a" => 1,
    ///     "b" => map_value! {
    ///         "x" => 2,
    ///     }
    /// };
    ///
    /// let _ = v.insert_by_path("b.x", Value::from(2));
    ///
    /// assert_eq!(v, expected);
    /// ```
    pub fn insert_by_path(&mut self, path: &str, v: Value) -> Option<Value> {
        if path == "" {
            return None;
        }
        let tokens = path.split('.').skip(1);
        let mut last_key = path.split('.').next().unwrap();
        let mut target = self;

        for token in tokens {
            let target_once = target;
            let target_opt = match *target_once {
                Value::Map(ref mut map) => {
                    Some(map.get_or_insert(last_key.to_string(), Value::from(Map::new())))
                }
                ref mut other => {
                    *other = Value::Map(Map::new());
                    Some(other)
                }
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
            last_key = token;
        }

        match target {
            Value::Map(ref mut map) => map.insert(last_key.to_string(), v),
            other => {
                let mut map = Map::new();
                map.insert(last_key.to_string(), v);
                *other = Value::Map(map);
                None
            }
        }
    }

    pub fn parse_integer<T>(&self) -> Option<T>
    where
        T: FromStr + FromPrimitive,
        Value: From<T>,
    {
        match self {
            Value::String(ref s) => s.parse::<T>().ok(),
            Value::U8(v) => T::from_u8(*v),
            Value::U16(v) => T::from_u16(*v),
            Value::U32(v) => T::from_u32(*v),
            Value::U64(v) => T::from_u64(*v),
            Value::I8(v) => T::from_i8(*v),
            Value::I16(v) => T::from_i16(*v),
            Value::I32(v) => T::from_i32(*v),
            Value::I64(v) => T::from_i64(*v),
            Value::Some(v) => Value::parse_integer::<T>(v),
            _ => None,
        }
    }

    pub fn parse_f32(&self) -> Option<f32> {
        match self {
            Value::String(ref s) => s.parse::<f32>().ok(),
            Value::U8(v) => f32::from_u8(*v),
            Value::U16(v) => f32::from_u16(*v),
            Value::U32(v) => f32::from_u32(*v),
            Value::U64(v) => f32::from_u64(*v),
            Value::I8(v) => f32::from_i8(*v),
            Value::I16(v) => f32::from_i16(*v),
            Value::I32(v) => f32::from_i32(*v),
            Value::I64(v) => f32::from_i64(*v),
            Value::F32(v) => Some(*v),
            Value::F64(v) => Some(*v as f32),
            Value::Some(v) => Value::parse_f32(v),
            _ => None,
        }
    }

    pub fn parse_f64(&self) -> Option<f64> {
        match self {
            Value::String(ref s) => s.parse::<f64>().ok(),
            Value::U8(v) => f64::from_u8(*v),
            Value::U16(v) => f64::from_u16(*v),
            Value::U32(v) => f64::from_u32(*v),
            Value::U64(v) => f64::from_u64(*v),
            Value::I8(v) => f64::from_i8(*v),
            Value::I16(v) => f64::from_i16(*v),
            Value::I32(v) => f64::from_i32(*v),
            Value::I64(v) => f64::from_i64(*v),
            Value::F32(v) => Some(*v as f64),
            Value::F64(v) => Some(*v),
            Value::Some(v) => Value::parse_f64(v),
            _ => None,
        }
    }

    pub fn parse_str(&self) -> Option<String> {
        match self {
            Value::Bool(v) => Some(if *v {
                "true".to_string()
            } else {
                "false".to_string()
            }),
            Value::String(ref s) => Some(s.to_string()),
            Value::Bytes(bytes) => std::str::from_utf8(bytes.as_slice())
                .map(|x| x.to_string())
                .ok(),
            Value::U8(v) => parse_str_from_integer(*v),
            Value::U16(v) => parse_str_from_integer(*v),
            Value::U32(v) => parse_str_from_integer(*v),
            Value::U64(v) => parse_str_from_integer(*v),
            Value::I8(v) => parse_str_from_integer(*v),
            Value::I16(v) => parse_str_from_integer(*v),
            Value::I32(v) => parse_str_from_integer(*v),
            Value::I64(v) => parse_str_from_integer(*v),
            Value::F32(v) => parse_str_from_float(*v),
            Value::F64(v) => parse_str_from_float(*v),
            Value::Some(v) => Value::parse_str(v),
            _ => None,
        }
    }
}

#[inline]
fn parse_str_from_integer<T: itoa::Integer>(v: T) -> Option<String> {
    let mut s = String::new();
    itoa::fmt(&mut s, v).map(|_| s).ok()
}

#[inline]
fn parse_str_from_float<T: ryu::Float>(v: T) -> Option<String> {
    let mut buf = ryu::Buffer::new();
    Some(buf.format(v).to_string())
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl Default for Value {
    fn default() -> Self {
        Value::None
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Map(map) => map.fmt(f),
            Value::Seq(vec) => {
                let mut first = true;
                f.write_str("[")?;
                for v in vec {
                    if first {
                        first = false;
                    } else {
                        f.write_str(", ")?;
                    }
                    f.write_fmt(format_args!("{}", v))?;
                }
                f.write_str("]")
            }
            Value::Unit => f.write_str("Unit"),
            Value::None => f.write_str("None"),
            _ => f.write_str(&self.parse_str().unwrap_or_else(|| "".to_string())),
        }
    }
}

///////////////////////////////////////////
// from ///////////////////////////////////
///////////////////////////////////////////

macro_rules! impl_from_to_value {
    ($t:ident, $variant: ident) => {
        impl From<$t> for Value {
            fn from(v: $t) -> Self {
                Value::$variant(v)
            }
        }
    };
}

impl_from_to_value!(bool, Bool);
impl_from_to_value!(u8, U8);
impl_from_to_value!(u16, U16);
impl_from_to_value!(u32, U32);
impl_from_to_value!(u64, U64);
impl_from_to_value!(i8, I8);
impl_from_to_value!(i16, I16);
impl_from_to_value!(i32, I32);
impl_from_to_value!(i64, I64);
impl_from_to_value!(f32, F32);
impl_from_to_value!(f64, F64);
impl_from_to_value!(String, String);

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Value::Bytes(Vec::from(v))
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::String(v.to_string())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<Map<String, Value>> for Value {
    fn from(v: Map<String, Value>) -> Self {
        Value::Map(v)
    }
}

impl From<&mut Map<String, Value>> for Value {
    fn from(v: &mut Map<String, Value>) -> Self {
        Value::Map(v.clone())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Seq(v)
    }
}

impl From<&mut Vec<Value>> for Value {
    fn from(v: &mut Vec<Value>) -> Self {
        Value::Seq(v.clone())
    }
}

impl From<Duration> for Value {
    fn from(v: Duration) -> Self {
        Value::TimeStamp(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(r) => Value::Some(Box::new(r.into())),
            None => Value::None,
        }
    }
}

////////////////////////////////////////////////
// partial_eq///////////////////////////////////
////////////////////////////////////////////////

////////////////////////////////////////////////
// string / str
impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        self.as_str().map_or(false, |x| x == other)
    }
}

impl<'a> PartialEq<&'a str> for Value {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().map_or(false, |x| x == *other)
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        other.as_str().map_or(false, |x| x == self)
    }
}

impl<'a> PartialEq<Value> for &'a str {
    fn eq(&self, other: &Value) -> bool {
        other.as_str().map_or(false, |x| x == *self)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        self.as_str().map_or(false, |x| x == other.as_str())
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        other.as_str().map_or(false, |x| x == self.as_str())
    }
}

////////////////////////////////////////////////
// duration

impl PartialEq<Duration> for Value {
    fn eq(&self, other: &Duration) -> bool {
        self.as_timestamp().map_or(false, |x| x == *other)
    }
}

impl PartialEq<Value> for Duration {
    fn eq(&self, other: &Value) -> bool {
        other.as_timestamp().map_or(false, |x| x == *self)
    }
}

////////////////////////////////////////////////
// number

macro_rules! impl_partial_eq_number {
    ($t:ident, $variant: ident) => {
        impl PartialEq<$t> for Value {
            fn eq(&self, other: &$t) -> bool {
                match *self {
                    Value::$variant(ref v) => *v == *other,
                    _ => false,
                }
            }
        }

        impl PartialEq<Value> for $t {
            fn eq(&self, other: &Value) -> bool {
                match *other {
                    Value::$variant(ref v) => *v == *self,
                    _ => false,
                }
            }
        }

        impl<'a> PartialEq<$t> for &'a Value {
            fn eq(&self, other: &$t) -> bool {
                match *self {
                    Value::$variant(ref v) => *v == *other,
                    _ => false,
                }
            }
        }

        impl<'a> PartialEq<$t> for &'a mut Value {
            fn eq(&self, other: &$t) -> bool {
                match *self {
                    Value::$variant(ref v) => *v == *other,
                    _ => false,
                }
            }
        }
    };
}

impl_partial_eq_number!(u8, U8);
impl_partial_eq_number!(u16, U16);
impl_partial_eq_number!(u32, U32);
impl_partial_eq_number!(u64, U64);

impl_partial_eq_number!(i8, I8);
impl_partial_eq_number!(i16, I16);
impl_partial_eq_number!(i32, I32);
impl_partial_eq_number!(i64, I64);

impl_partial_eq_number!(f32, F32);
impl_partial_eq_number!(f64, F64);

impl_partial_eq_number!(bool, Bool);

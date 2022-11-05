use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use toy_map::Map;
use toy_pack::FromPrimitive;

/// The value itself is represented by a scalar, key-value pair, array, etc.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Bytes(Vec<u8>),
    None,
    Seq(Vec<Value>),
    Map(Map<String, Value>),
    TimeStamp(DateTime<Utc>),
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

    pub fn is_vec(&self) -> bool {
        self.as_vec().is_some()
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Seq(ref vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_true(&self) -> bool {
        match *self {
            Value::Bool(v) => v,
            _ => false,
        }
    }

    pub fn is_false(&self) -> bool {
        match *self {
            Value::Bool(v) => !v,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match *self {
            Value::Integer(_) => true,
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
            Value::Integer(v) => u32::from_i64(v),
            _ => None,
        }
    }

    pub fn is_u64(&self) -> bool {
        self.as_u64().is_some()
    }

    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Integer(v) => u64::from_i64(v),
            _ => None,
        }
    }

    pub fn is_bytes(&self) -> bool {
        self.as_bytes().is_some()
    }

    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        match *self {
            Value::Bytes(ref bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn is_timestamp(&self) -> bool {
        self.as_timestamp().is_some()
    }

    pub fn as_timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            Value::TimeStamp(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn is_same_type(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Bool(_), Value::Bool(_)) => true,
            (Value::Integer(_), Value::Integer(_)) => true,
            (Value::Number(_), Value::Number(_)) => true,
            (Value::String(_), Value::String(_)) => true,
            (Value::Bytes(_), Value::Bytes(_)) => true,
            (Value::None, Value::None) => true,
            (Value::Seq(_), Value::Seq(_)) => true,
            (Value::Map(_), Value::Map(_)) => true,
            (Value::TimeStamp(_), Value::TimeStamp(_)) => true,
            _ => false,
        }
    }

    pub fn as_same_type(&self, other: &Value) -> Option<Value> {
        match other {
            Value::Bool(_) => self.as_bool().map(Value::from),
            Value::Integer(_) => self.parse_integer::<i64>().map(Value::from),
            Value::Number(_) => self.parse_f64().map(Value::from),
            Value::String(_) => self.parse_str().map(Value::from),
            Value::Bytes(_) => self.as_bytes().map(Value::from),
            Value::None => match self {
                Value::None => Some(Value::None),
                _ => None,
            },
            Value::Seq(_) => self.as_vec().map(Value::from).map(Value::from),
            Value::Map(_) => self.as_map().map(Value::from),
            Value::TimeStamp(_) => self.parse_timestamp().map(Value::from),
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
            Value::Integer(v) => T::from_i64(*v),
            _ => None,
        }
    }

    pub fn parse_f32(&self) -> Option<f32> {
        match self {
            Value::String(ref s) => s.parse::<f32>().ok(),
            Value::Integer(v) => f32::from_i64(*v),
            Value::Number(v) => Some(*v as f32),
            _ => None,
        }
    }

    pub fn parse_f64(&self) -> Option<f64> {
        match self {
            Value::String(ref s) => s.parse::<f64>().ok(),
            Value::Integer(v) => f64::from_i64(*v),
            Value::Number(v) => Some(*v as f64),
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
            Value::Integer(v) => parse_str_from_integer(*v),
            Value::Number(v) => parse_str_from_float(*v),
            Value::TimeStamp(v) => Some(v.to_rfc3339()),
            _ => None,
        }
    }

    pub fn parse_timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            Value::TimeStamp(v) => Some(v.clone()),
            Value::String(v) => match DateTime::parse_from_rfc3339(v) {
                Ok(dt) => Some(dt.with_timezone(&Utc)),
                Err(_) => None,
            },
            _ => None,
        }
    }
}

#[inline]
fn parse_str_from_integer<T: itoa::Integer>(v: T) -> Option<String> {
    let mut buf = itoa::Buffer::new();
    Some(buf.format(v).to_string())
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
            Value::None => f.write_str("None"),
            _ => f.write_str(&self.parse_str().unwrap_or_else(|| "".to_string())),
        }
    }
}

///////////////////////////////////////////
// from ///////////////////////////////////
///////////////////////////////////////////

macro_rules! impl_from_to_value_integer {
    ($t:ident, $expr: ident) => {
        impl From<$t> for Value {
            fn from(v: $t) -> Self {
                let r = i64::$expr(v);
                match r {
                    Some(i) => Value::Integer(i),
                    None => Value::None,
                }
            }
        }
    };
}

impl_from_to_value_integer!(u8, from_u8);
impl_from_to_value_integer!(u16, from_u16);
impl_from_to_value_integer!(u32, from_u32);
impl_from_to_value_integer!(u64, from_u64);
impl_from_to_value_integer!(i8, from_i8);
impl_from_to_value_integer!(i16, from_i16);
impl_from_to_value_integer!(i32, from_i32);
impl_from_to_value_integer!(i64, from_i64);

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
    Value: From<T>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => Value::from(v),
            None => Value::None,
        }
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Value::from(v as i64)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Number(v as f64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Number(v)
    }
}

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

impl From<&Vec<u8>> for Value {
    fn from(v: &Vec<u8>) -> Self {
        Value::Bytes(v.clone())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
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

impl From<&Map<String, Value>> for Value {
    fn from(v: &Map<String, Value>) -> Self {
        Value::Map(v.clone())
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

impl From<&Vec<Value>> for Value {
    fn from(v: &Vec<Value>) -> Self {
        Value::Seq(v.clone())
    }
}

impl From<&mut Vec<Value>> for Value {
    fn from(v: &mut Vec<Value>) -> Self {
        Value::Seq(v.clone())
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(v: DateTime<Utc>) -> Self {
        Value::TimeStamp(v)
    }
}

impl From<&DateTime<Utc>> for Value {
    fn from(v: &DateTime<Utc>) -> Self {
        Value::TimeStamp(v.clone())
    }
}

////////////////////////////////////////////////
// Eq

impl Eq for Value {}

////////////////////////////////////////////////
// Ord

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::Number(v) => {
                if v.is_sign_negative() && v.is_infinite() {
                    Ordering::Less
                } else if v.is_infinite() {
                    Ordering::Greater
                } else if v.is_nan() {
                    Ordering::Less
                } else {
                    self.partial_cmp(other).unwrap()
                }
            }
            _ => self.partial_cmp(other).unwrap(),
        }
    }
}

////////////////////////////////////////////////
// bool
impl PartialEq<bool> for Value {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool().map_or(false, |x| x == *other)
    }
}

impl PartialEq<Value> for bool {
    fn eq(&self, other: &Value) -> bool {
        other.as_bool().map_or(false, |x| x == *self)
    }
}

impl<'a> PartialEq<bool> for &'a Value {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool().map_or(false, |x| x == *other)
    }
}

impl<'a> PartialEq<bool> for &'a mut Value {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool().map_or(false, |x| x == *other)
    }
}

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
// vec

impl PartialEq<Value> for Vec<Value> {
    fn eq(&self, other: &Value) -> bool {
        other.as_vec().map_or(false, |x| x == self)
    }
}

impl PartialEq<Vec<Value>> for Value {
    fn eq(&self, other: &Vec<Value>) -> bool {
        self.as_vec().map_or(false, |x| x == other)
    }
}

////////////////////////////////////////////////
// map

impl PartialEq<Value> for Map<String, Value> {
    fn eq(&self, other: &Value) -> bool {
        other.as_map().map_or(false, |x| x == self)
    }
}

impl PartialEq<Map<String, Value>> for Value {
    fn eq(&self, other: &Map<String, Value>) -> bool {
        self.as_map().map_or(false, |x| x == other)
    }
}

////////////////////////////////////////////////
// bytes

impl PartialEq<Vec<u8>> for Value {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.as_bytes().map_or(false, |x| x == other)
    }
}

impl PartialEq<Value> for Vec<u8> {
    fn eq(&self, other: &Value) -> bool {
        other.as_bytes().map_or(false, |x| x == self)
    }
}

////////////////////////////////////////////////
// datetime

impl PartialEq<DateTime<Utc>> for Value {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.as_timestamp().map_or(false, |x| x == *other)
    }
}

impl PartialEq<Value> for DateTime<Utc> {
    fn eq(&self, other: &Value) -> bool {
        other.as_timestamp().map_or(false, |x| x == *self)
    }
}

////////////////////////////////////////////////
// integer & number

macro_rules! impl_partial_eq_integer {
    ($t:ident, $variant: ident, $to: ident, $expr: ident) => {
        impl PartialEq<$t> for Value {
            fn eq(&self, other: &$t) -> bool {
                let i = $to::$expr(*other);
                i.is_some()
                    && match *self {
                        Value::$variant(ref v) => *v == i.unwrap(),
                        _ => false,
                    }
            }
        }

        impl PartialEq<Value> for $t {
            fn eq(&self, other: &Value) -> bool {
                let i = $to::$expr(*self);
                i.is_some()
                    && match *other {
                        Value::$variant(ref v) => *v == i.unwrap(),
                        _ => false,
                    }
            }
        }

        impl<'a> PartialEq<$t> for &'a Value {
            fn eq(&self, other: &$t) -> bool {
                let i = $to::$expr(*other);
                i.is_some()
                    && match *self {
                        Value::$variant(ref v) => *v == i.unwrap(),
                        _ => false,
                    }
            }
        }

        impl<'a> PartialEq<$t> for &'a mut Value {
            fn eq(&self, other: &$t) -> bool {
                let i = $to::$expr(*other);
                i.is_some()
                    && match *self {
                        Value::$variant(ref v) => *v == i.unwrap(),
                        _ => false,
                    }
            }
        }
    };
}

macro_rules! impl_partial_eq_number {
    ($t:ident, $variant: ident) => {
        impl PartialEq<$t> for Value {
            fn eq(&self, other: &$t) -> bool {
                match *self {
                    Value::$variant(ref v) => *v == *other as f64,
                    _ => false,
                }
            }
        }

        impl PartialEq<Value> for $t {
            fn eq(&self, other: &Value) -> bool {
                match *other {
                    Value::$variant(ref v) => *v == *self as f64,
                    _ => false,
                }
            }
        }

        impl<'a> PartialEq<$t> for &'a Value {
            fn eq(&self, other: &$t) -> bool {
                match *self {
                    Value::$variant(ref v) => *v == *other as f64,
                    _ => false,
                }
            }
        }

        impl<'a> PartialEq<$t> for &'a mut Value {
            fn eq(&self, other: &$t) -> bool {
                match *self {
                    Value::$variant(ref v) => *v == *other as f64,
                    _ => false,
                }
            }
        }
    };
}

impl_partial_eq_integer!(u8, Integer, i64, from_u8);
impl_partial_eq_integer!(u16, Integer, i64, from_u16);
impl_partial_eq_integer!(u32, Integer, i64, from_u32);
impl_partial_eq_integer!(u64, Integer, i64, from_u64);

impl_partial_eq_integer!(i8, Integer, i64, from_i8);
impl_partial_eq_integer!(i16, Integer, i64, from_i16);
impl_partial_eq_integer!(i32, Integer, i64, from_i32);
impl_partial_eq_integer!(i64, Integer, i64, from_i64);

impl_partial_eq_number!(f32, Number);
impl_partial_eq_number!(f64, Number);

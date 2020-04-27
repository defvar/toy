use super::{Error, SerializeMapOps, SerializeSeqOps, SerializeStructOps};
use crate::ser::SerializeTupleVariantOps;

/// The traits that the serializable data structure implements.
/// Several primitive types "impl" are provided by default.
///
/// 直列化可能データ構造が実装するトレイト
/// いくつかのプリミティブ型 "impl" がデフォルトで提供されています
///
pub trait Serializable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

/// The traits for serialization processing.
///
/// シリアライズ処理を行うトレイト。
///
pub trait Serializer: Sized {
    type Ok;
    type Error: Error;
    type SeqAccessOps: SerializeSeqOps<Ok = Self::Ok, Error = Self::Error>;
    type MapAccessOps: SerializeMapOps<Ok = Self::Ok, Error = Self::Error>;
    type StructAccessOps: SerializeStructOps<Ok = Self::Ok, Error = Self::Error>;
    type TupleVariantOps: SerializeTupleVariantOps<Ok = Self::Ok, Error = Self::Error>;

    /// Serialize a `bool`.
    ///
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u8`.
    ///
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u16`.
    ///
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u32`.
    ///
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u64`.
    ///
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `i8`.
    ///
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `i16`.
    ///
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `i32`.
    ///
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `i64`.
    ///
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `f32`.
    ///
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `f64`.
    ///
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `char`
    ///
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `&str`.
    ///
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;

    /// Serialize a seq.
    ///
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SeqAccessOps, Self::Error>;

    /// Serialize a map.
    ///
    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapAccessOps, Self::Error>;

    /// Serialize the structure.
    /// Depending on the specification of the serialization format, it may be a sequence or a map.
    ///
    /// 構造体のシリアライズを行います。
    /// シリアライズフォーマットの仕様によって、シーケンスだったりマップだったりするでしょう。
    ///
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::StructAccessOps, Self::Error>;

    /// unit variant like `E::A` in `enum E { A, B }`.
    ///
    /// ```edition2018
    /// enum E {
    ///     A,
    ///     B,
    /// }
    /// ```
    fn serialize_unit_variant(
        self,
        name: &'static str,
        idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;

    /// newtype variant like `E::A(123)` in `enum E { A(u32), B(String), }`
    ///
    /// ```edition2018
    /// enum E {
    ///     A(u32),
    ///     B(String),
    /// }
    /// ```
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        idx: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable;

    /// tuple variant like `E::A(123, 456)` in `enum E { A(u32, u32), B(u32, String), }`
    ///
    /// ```edition2018
    ///  enum E {
    ///      A(u32, u32),
    ///      B(u32, String),
    ///  }
    /// ```
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::TupleVariantOps, Self::Error>;

    /// Serialize a [`Some(T)`] value.
    ///
    /// [`Some(T)`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable;

    /// Serialize a [`None`] value.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;

    /// Collect an iterator as a sequence.
    /// The default implementation using `serialize_seq`.
    ///
    /// シーケンスの要素をシリアライズします。
    /// デフォルト実装は `serialize_seq` を行います。
    ///
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        I::Item: Serializable,
    {
        let iter = iter.into_iter();
        let len = match iter.size_hint() {
            (lo, Some(hi)) if lo == hi => Some(lo),
            _ => None,
        };
        let mut ser = self.serialize_seq(len)?;
        for item in iter {
            ser.next(&item)?;
        }
        ser.end()
    }

    /// Collect an iterator as a map.
    /// The default implementation using `serialize_map`.
    ///
    /// マップの要素をシリアライズします。
    /// デフォルト実装は `serialize_map` を行います。
    ///
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serializable,
        V: Serializable,
        I: IntoIterator<Item = (K, V)>,
    {
        let iter = iter.into_iter();
        let len = match iter.size_hint() {
            (lo, Some(hi)) if lo == hi => Some(lo),
            _ => None,
        };
        let mut ser = self.serialize_map(len)?;
        for (k, v) in iter {
            ser.next_key(&k)?;
            ser.next_value(&v)?;
        }
        ser.end()
    }
}

impl<'a, T: ?Sized> Serializable for &'a T
where
    T: Serializable,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<'a, T: ?Sized> Serializable for &'a mut T
where
    T: Serializable,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

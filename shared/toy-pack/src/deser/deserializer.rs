use super::{Error, Visitor};

/// The traits that the serializable data structure implements.
/// Several primitive types "impl" are provided by default.
///
/// 直列化可能データ構造が実装するトレイト
/// いくつかのプリミティブ型 "impl" がデフォルトで提供されています
///
pub trait Deserializable<'toy>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>;
}

pub trait DeserializableCore<'toy>: Sized {
    type Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'toy>;
}

/// A data structure that can be deserialized without borrowing any data from the deserializer.
///
/// デシリアライザからデータを借りることなくデシリアライズできるデータ構造が実装するトレイト
///
pub trait DeserializableOwned: for<'de> Deserializable<'de> {}

impl<T> DeserializableOwned for T where T: for<'de> Deserializable<'de> {}

/// The traits for deserialization processing.
///
/// デシリアライズ処理を行うトレイト。
///
pub trait Deserializer<'toy>: Sized {
    type Error: Error;

    /// Deserialize `bool`
    ///
    fn deserialize_bool(self) -> Result<bool, Self::Error>;

    /// Deserialize `u8`
    ///
    fn deserialize_u8(self) -> Result<u8, Self::Error>;

    /// Deserialize `u16`
    ///
    fn deserialize_u16(self) -> Result<u16, Self::Error>;

    /// Deserialize `u32`
    ///
    fn deserialize_u32(self) -> Result<u32, Self::Error>;

    /// Deserialize `u64`
    ///
    fn deserialize_u64(self) -> Result<u64, Self::Error>;

    /// Deserialize `i8`
    ///
    fn deserialize_i8(self) -> Result<i8, Self::Error>;

    /// Deserialize `i16`
    ///
    fn deserialize_i16(self) -> Result<i16, Self::Error>;

    /// Deserialize `i32`
    ///
    fn deserialize_i32(self) -> Result<i32, Self::Error>;

    /// Deserialize `i64`
    ///
    fn deserialize_i64(self) -> Result<i64, Self::Error>;

    /// Deserialize `f32`
    ///
    fn deserialize_f32(self) -> Result<f32, Self::Error>;

    /// Deserialize `f64`
    ///
    fn deserialize_f64(self) -> Result<f64, Self::Error>;

    /// Deserialize `char`
    ///
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize `&str`
    ///
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize `String`
    ///
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize the sequence.
    ///
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize the map.
    ///
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize the structure.
    /// Depending on the specification of the serialization format, it may be a sequence or a map.
    ///
    /// 構造体のデシリアライズを行います。
    /// シリアライズフォーマットの仕様によって、シーケンスだったりマップだったりするでしょう。
    ///
    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize `enum`.
    ///
    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize `Option`.
    /// Depending on the specification of the serialization format, the treatment of `Some (v)` and `None` will be different.
    ///
    /// `Option`のデシリアライズを行います。
    /// シリアライズフォーマットの仕様によって、`Some(v)`や`None`の扱いは異なるでしょう。
    ///
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize any value.
    ///
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;

    /// Deserialize and discard the value.
    /// It is used when deserialization target is an unknown key etc.
    ///
    /// 値を破棄します。
    /// デシリアライズ対象が未知のキーだった場合等に利用します。
    ///
    fn discard<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>;
}

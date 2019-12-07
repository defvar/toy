use super::{Deserializable, Error, Visitor};

/// Provides a `Visitor` access to each element of a sequence in the input.
///
/// シーケンスの各要素へのアクセスを提供します。
///
pub trait DeserializeSeqOps<'toy> {
    type Error: Error;

    /// Returns for the next value in the sequence.
    ///
    fn next<T>(&mut self) -> Result<Option<T::Value>, Self::Error> where T: Deserializable<'toy>;

    /// Returns the number of value remaining in the sequence, if known.
    ///
    fn size_hint(&self) -> Option<usize>;
}

/// Provides a `Visitor` access to each element of a map in the input.
///
/// マップの各要素へのアクセスを提供します。
///
pub trait DeserializeMapOps<'toy> {
    type Error: Error;

    /// Returns for the field identifier in the map.
    /// Depending on the serialization specification, it may be numeric or a string of field names.
    ///
    /// 構造体のフィールドを識別する値を取得します。
    /// シリアライズ仕様によって、数値だったりフィールド名の文字列だったりするでしょう。
    ///
    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error> where V: Visitor<'toy>;

    /// Returns for the next key in the map.
    ///
    fn next_key<T>(&mut self) -> Result<Option<T::Value>, Self::Error> where T: Deserializable<'toy>;

    /// Returns for the next value in the map.
    ///
    fn next_value<T>(&mut self) -> Result<T::Value, Self::Error> where T: Deserializable<'toy>;

    /// Returns the number of entries remaining in the map, if known.
    ///
    fn size_hint(&self) -> Option<usize>;
}

/// Provides a `Visitor` access to variant of a enum in the input.
///
/// enumの各ヴァリアントへのアクセスを提供します。
///
pub trait DeserializeVariantOps<'toy>: Sized {
    type Error: Error;

    /// Called when get variant identifier.
    ///
    /// enumのヴァリアントを識別する値を取得する際に使用します。
    ///
    fn variant_identifier<V>(self, visitor: V) -> Result<(V::Value, Self), Self::Error>
        where V: Visitor<'toy>;

    /// Called when deserializing a variant with no values.
    ///
    /// 値の無いヴァリアントをデシリアライズする場合に使用します。
    ///
    fn unit_variant(self) -> Result<(), Self::Error>;

    /// Called when deserializing a variant with a single value.
    ///
    /// 1つの値を持つヴァリアントをデシリアライズする場合に使用します。
    ///
    fn newtype_variant<T>(self) -> Result<T::Value, Self::Error> where T: Deserializable<'toy>;

    /// Called when deserializing tuple variant.
    ///
    /// タプル形式のヴァリアントをデシリアライズする場合に使用します。
    ///
    fn tuple_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'toy>;
}

use super::{Deserializable, Deserializer, Error, Visitor};

/// It is an alternative type that is used when deserialization target is an unknown key.
/// Deserialized information is not retained.
///
/// デシリアライズ対象が未知のキーだった場合等に利用される代替の型です。
/// デシリアライズされた情報は保持されません。
/// デフォルトで提供される `Deserializable` の実装では 'deserializer.discard' を呼び出します。
///
pub struct Discard;

struct DiscardVisitor;

impl<'a> Visitor<'a> for DiscardVisitor {
    type Value = Discard;

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'a>,
    {
        Discard::deserialize(deserializer)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Discard)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Discard)
    }
}

impl<'toy: 'a, 'a> Deserializable<'toy> for Discard {
    type Value = Discard;

    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.discard(DiscardVisitor)
    }
}

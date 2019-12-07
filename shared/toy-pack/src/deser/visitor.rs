use super::{DeserializeMapOps, Deserializer, DeserializeSeqOps, DeserializeVariantOps, Error};

/// The trait to scan target values​during deserialization.
/// The default is an error.
/// Please use the visitor who implemented the necessary implementation according to the deserialization value.
///
/// デシリアライズ時に対象の値を走査するためのトレイト。
/// デフォルトではエラーとなります。
/// デシリアライズの値に合わせて必要な実装を行ったvisitorを利用してください。
///
/// # Example
///
/// ```edition2018
/// # use toy_pack::deser::{Visitor, Error};
///
/// struct StrVisitor;
///
/// impl<'a> Visitor<'a> for StrVisitor {
///     type Value = &'a str;
///
///     fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E> where E: Error {
///         Ok(v)
///     }
/// }
/// ```
pub trait Visitor<'a>: Sized {
    type Value;

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: Error {
        let _ = v;
        Err(Error::invalid_type("u32"))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        let _ = v;
        Err(Error::invalid_type("str"))
    }

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E> where E: Error {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        self.visit_str(&v)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where A: DeserializeSeqOps<'a> {
        let _ = seq;
        Err(Error::invalid_type("seq"))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where A: DeserializeMapOps<'a> {
        let _ = map;
        Err(Error::invalid_type("map"))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where A: DeserializeVariantOps<'a> {
        let _ = data;

        Err(Error::invalid_type("enum"))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'a>
    {
        let _ = deserializer;
        Err(Error::invalid_type("option"))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
        Err(Error::invalid_type("option"))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where E: Error {
        Err(Error::invalid_type("unit"))
    }
}

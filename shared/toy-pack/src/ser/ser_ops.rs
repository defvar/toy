use super::{Error, Serializable};

/// Provides access to each element of a sequence to output.
///
/// シーケンスの各要素へのアクセスを提供します。
///
pub trait SerializeSeqOps {
    type Ok;

    type Error: Error;

    fn next<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Provides access to each element of a map to output.
///
/// マップの各要素へのアクセスを提供します。
///
pub trait SerializeMapOps {
    type Ok;

    type Error: Error;

    fn next_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serializable;

    fn next_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Provides access to each field of a struct to output.
///
/// 構造体の各フィールドへのアクセスを提供します。
///
pub trait SerializeStructOps {
    type Ok;

    type Error: Error;

    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Provides access to each element of a tuple variant to output.
///
pub trait SerializeTupleVariantOps {
    type Ok;

    type Error: Error;

    fn next<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

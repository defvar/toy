use std::fmt::Display;

/// This Trait using `Serializer`.
/// It is used when an error occurs in the implementation of serialization.
///
/// シリアライザで利用されるトレイト。
/// シリアライズの実装でエラーが発生した場合に利用されます。
///
pub trait Error: Sized {
    fn custom<T>(msg: T) -> Self
    where
        T: Display;
}

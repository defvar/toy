use std::fmt::Display;

use failure::Fail;

/// This Trait using `Deserializer`.
/// It is used when an error occurs in the implementation of deserialization.
///
/// デシリアライザで利用されるトレイト。
/// デシリアライズの実装でエラーが発生した場合に利用されます。
///
pub trait Error: Sized + Fail {
    fn custom<T>(msg: T) -> Self where T: Display;

    fn invalid_type<T>(expected_type_name: T) -> Self where T: Display {
        Error::custom(format_args!("invalid type: unexpected {}", expected_type_name))
    }

    fn invalid_value<T, T2>(unexpected: T, expected: T2) -> Self where T: Display, T2: Display {
        Error::custom(format_args!("invalid value: {}, expected {}", unexpected, expected))
    }

    fn invalid_length(len: usize) -> Self {
        Error::custom(format_args!("invalid length. len:{}", len))
    }

    fn duplicate_field(name: &str) -> Self {
        Error::custom(format_args!("duplicate field. name:{}", name))
    }

    fn unknown_variant(name: &str) -> Self {
        Error::custom(format_args!("unknown variant for {}", name))
    }
}

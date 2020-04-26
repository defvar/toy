use super::{Serializable, Serializer};

macro_rules! primitive_serializer_impl {
    ($t: ident, $method: ident) => {
        impl Serializable for $t {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.$method(*self)
            }
        }
    };
}

impl Serializable for str {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl Serializable for String {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl Serializable for char {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_char(*self)
    }
}

impl Serializable for usize {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(*self as u64)
    }
}

impl Serializable for isize {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}

primitive_serializer_impl!(u8, serialize_u8);
primitive_serializer_impl!(u16, serialize_u16);
primitive_serializer_impl!(u32, serialize_u32);
primitive_serializer_impl!(u64, serialize_u64);
primitive_serializer_impl!(i8, serialize_i8);
primitive_serializer_impl!(i16, serialize_i16);
primitive_serializer_impl!(i32, serialize_i32);
primitive_serializer_impl!(i64, serialize_i64);
primitive_serializer_impl!(f32, serialize_f32);
primitive_serializer_impl!(f64, serialize_f64);
primitive_serializer_impl!(bool, serialize_bool);

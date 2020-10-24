use super::{from_primitive::FromPrimitive, Deserializable, Deserializer, Error, Visitor};

////////////////////////////////////////////////////

struct StrVisitor;

impl<'a> Visitor<'a> for StrVisitor {
    type Value = &'a str;

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        std::str::from_utf8(v).map_err(|_| Error::invalid_value("[borrowed bytes]", "str"))
    }
}

impl<'toy: 'a, 'a> Deserializable<'toy> for &'a str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_str(StrVisitor)
    }
}

////////////////////////////////////////////////////

struct StringVisitor;

impl<'a> Visitor<'a> for StringVisitor {
    type Value = String;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => Ok(s.to_owned()),
            Err(_) => Err(Error::invalid_value("[borrowed bytes]", "str")),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::invalid_value("[borrowed bytes]", "str")),
        }
    }
}

impl<'toy> Deserializable<'toy> for String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_string(StringVisitor)
    }
}

////////////////////////////////////////////////////

struct CharVisitor;

impl<'a> Visitor<'a> for CharVisitor {
    type Value = char;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut iter = v.chars();
        match (iter.next(), iter.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(Error::invalid_type("char")),
        }
    }
}

impl<'toy> Deserializable<'toy> for char {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_char(CharVisitor)
    }
}

////////////////////////////////////////////////////////////////////////////////

struct BytesVisitor;

impl<'a> Visitor<'a> for BytesVisitor {
    type Value = &'a [u8];

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.as_bytes())
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}

impl<'toy: 'a, 'a> Deserializable<'toy> for &'a [u8] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        deserializer.deserialize_bytes(BytesVisitor)
    }
}

////////////////////////////////////////////////////

impl<'toy> Deserializable<'toy> for usize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        let v = deserializer.deserialize_u64()?;
        match FromPrimitive::from_u64(v) {
            Some(v) => Ok(v),
            None => Err(Error::invalid_type("usize")),
        }
    }
}

impl<'toy> Deserializable<'toy> for isize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'toy>,
    {
        let v = deserializer.deserialize_i64()?;
        match FromPrimitive::from_i64(v) {
            Some(v) => Ok(v),
            None => Err(Error::invalid_type("isize")),
        }
    }
}

////////////////////////////////////////////////////

macro_rules! primitive_serializer_impl {
    ($t: ident, $method: ident) => {
        impl<'toy> Deserializable<'toy> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'toy>,
            {
                deserializer.$method()
            }
        }
    };
}

primitive_serializer_impl!(u8, deserialize_u8);
primitive_serializer_impl!(u16, deserialize_u16);
primitive_serializer_impl!(u32, deserialize_u32);
primitive_serializer_impl!(u64, deserialize_u64);
primitive_serializer_impl!(i8, deserialize_i8);
primitive_serializer_impl!(i16, deserialize_i16);
primitive_serializer_impl!(i32, deserialize_i32);
primitive_serializer_impl!(i64, deserialize_i64);
primitive_serializer_impl!(f32, deserialize_f32);
primitive_serializer_impl!(f64, deserialize_f64);
primitive_serializer_impl!(bool, deserialize_bool);

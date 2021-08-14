//! Deserializer implementation.

use crate::deser_ops::{DeserializeMap, DeserializeVariant};
use crate::error::QueryParseError;
use std::borrow::Cow;
use toy_pack::deser::{Deserializer, Error, Visitor};

pub struct Parse<'a> {
    raw: DeserializeMap<'a, form_urlencoded::Parse<'a>>,
}

impl<'a> Parse<'a> {
    pub fn new(input: &'a [u8]) -> Parse<'a> {
        Parse {
            raw: DeserializeMap::new(form_urlencoded::parse(input)),
        }
    }
}

/// query part key or value.
pub(crate) struct Part<'a>(pub(crate) Cow<'a, str>);

macro_rules! forward_parsed_value {
    ($t: ident, $func: ident) => {
        fn $func(self) -> Result<$t, Self::Error> {
            match self.0.parse::<$t>() {
                Ok(val) => Ok(val),
                Err(e) => Err(Error::custom(e)),
            }
        }
    };
}

impl<'toy> Deserializer<'toy> for Part<'toy> {
    type Error = QueryParseError;

    forward_parsed_value!(bool, deserialize_bool);
    forward_parsed_value!(u8, deserialize_u8);
    forward_parsed_value!(u16, deserialize_u16);
    forward_parsed_value!(u32, deserialize_u32);
    forward_parsed_value!(u64, deserialize_u64);
    forward_parsed_value!(i8, deserialize_i8);
    forward_parsed_value!(i16, deserialize_i16);
    forward_parsed_value!(i32, deserialize_i32);
    forward_parsed_value!(i64, deserialize_i64);
    forward_parsed_value!(f32, deserialize_f32);
    forward_parsed_value!(f64, deserialize_f64);

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_enum(DeserializeVariant(self.0))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_unit()
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.0 {
            Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
            Cow::Owned(value) => visitor.visit_string(value),
        }
    }

    fn discard<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_none()
    }
}

impl<'toy> Deserializer<'toy> for Parse<'toy> {
    type Error = QueryParseError;

    fn deserialize_bool(self) -> Result<bool, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_u8(self) -> Result<u8, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_u16(self) -> Result<u16, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_u32(self) -> Result<u32, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_u64(self) -> Result<u64, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_i8(self) -> Result<i8, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_i16(self) -> Result<i16, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_i32(self) -> Result<i32, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_i64(self) -> Result<i64, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_f32(self) -> Result<f32, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_f64(self) -> Result<f64, Self::Error> {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_byte_buf<V>(
        self,
        _visitor: V,
    ) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_map(self.raw)
    }

    fn deserialize_struct<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(self, _visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'toy>>::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_map(visitor)
    }

    fn discard<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(QueryParseError::map_type_only())
    }
}

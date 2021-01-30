use crate::common::error::QueryParseError;
use core::marker::PhantomData;
use std::borrow::Cow;
use toy_pack::deser::{
    DeserializableCore, DeserializableOwned, DeserializeMapOps, DeserializeVariantOps,
    Deserializer, Error, Visitor,
};
use warp::Filter;

pub fn query_opt<T>() -> impl Filter<Extract = (Option<T>,), Error = warp::Rejection> + Clone
where
    T: DeserializableOwned + Send,
{
    query::<T>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<T>,), warp::Rejection>((None,)) })
}

pub fn query<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializableOwned + Send,
{
    warp::query::raw().and_then(|h: String| {
        tracing::debug!("query:{:?}", h);
        let r = T::deserialize(Parse {
            raw: DeserializeMap::new(form_urlencoded::parse(h.as_bytes())),
        })
        .map_err(|e| warp::reject::custom(e));
        std::future::ready(r)
    })
}

struct Parse<'a> {
    raw: DeserializeMap<'a, form_urlencoded::Parse<'a>>,
}

struct Part<'a>(Cow<'a, str>);

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

struct DeserializeMap<'a, I>
where
    I: Iterator<Item = (Cow<'a, str>, Cow<'a, str>)>,
{
    iter: I,
    value: Option<Cow<'a, str>>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, I> DeserializeMap<'a, I>
where
    I: Iterator<Item = (Cow<'a, str>, Cow<'a, str>)>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            value: None,
            lifetime: PhantomData,
        }
    }
}

impl<'toy, I> DeserializeMapOps<'toy> for DeserializeMap<'toy, I>
where
    I: Iterator<Item = (Cow<'toy, str>, Cow<'toy, str>)>,
{
    type Error = QueryParseError;

    fn next_identifier<V>(&mut self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                match k {
                    Cow::Borrowed(value) => visitor.visit_borrowed_str(value).map(Some),
                    Cow::Owned(value) => visitor.visit_string(value).map(Some),
                }
            }
            None => Ok(None),
        }
    }

    fn next_key_core<T>(&mut self, element: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                element.deserialize(Part(k)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_core<T>(&mut self, element: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        let value = self.value.take();
        let value = value.expect("MapAccess::visit_value called before visit_key");
        element.deserialize(Part(value))
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

struct DeserializeVariant<'toy>(Cow<'toy, str>);

impl<'toy> DeserializeVariantOps<'toy> for DeserializeVariant<'toy> {
    type Error = QueryParseError;

    fn variant_identifier<V>(self, visitor: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: Visitor<'toy>,
    {
        let s = self.0.clone();
        Ok((
            match self.0 {
                Cow::Borrowed(value) => visitor.visit_borrowed_str::<QueryParseError>(value),
                Cow::Owned(value) => visitor.visit_string::<QueryParseError>(value),
            }?,
            DeserializeVariant(s),
        ))
    }

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_core<T>(self, _core: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializableCore<'toy>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn tuple_variant<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        Err(Error::custom("expected unit variant"))
    }
}

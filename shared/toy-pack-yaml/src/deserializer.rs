use toy_pack::deser::from_primitive::FromPrimitive;
use toy_pack::deser::{Deserializer, Error, Visitor};

use super::decoder::{Decoder, Event};
use super::deser_ops::DeserializeCompound;
use super::error::YamlError;
use yaml_rust::Yaml;

impl<'toy, 'a> Deserializer<'toy> for &'a mut Decoder {
    type Error = YamlError;

    fn deserialize_bool(self) -> Result<bool, Self::Error> {
        self.decode_bool()
    }

    fn deserialize_u8(self) -> Result<u8, Self::Error> {
        u8::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("u8"))
    }

    fn deserialize_u16(self) -> Result<u16, Self::Error> {
        u16::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("u16"))
    }

    fn deserialize_u32(self) -> Result<u32, Self::Error> {
        u32::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("u32"))
    }

    fn deserialize_u64(self) -> Result<u64, Self::Error> {
        u64::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("u64"))
    }

    fn deserialize_i8(self) -> Result<i8, Self::Error> {
        i8::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("i8"))
    }

    fn deserialize_i16(self) -> Result<i16, Self::Error> {
        i16::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("i16"))
    }

    fn deserialize_i32(self) -> Result<i32, Self::Error> {
        i32::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("i32"))
    }

    fn deserialize_i64(self) -> Result<i64, Self::Error> {
        self.decode_int()
    }

    fn deserialize_f32(self) -> Result<f32, Self::Error> {
        f32::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("f32"))
    }

    fn deserialize_f64(self) -> Result<f64, Self::Error> {
        f64::from_i64(self.decode_int()?).ok_or_else(|| Error::invalid_type("f64"))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.decode_string()
            .map(|x| visitor.visit_str(x.as_str()))?
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_str(visitor)
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.next()?.0 {
            Event::SequenceStart => visitor.visit_seq(DeserializeCompound::new(self)),
            _ => Err(Error::invalid_type("seq")),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.next()?.0 {
            Event::MappingStart => visitor.visit_map(DeserializeCompound::new(self)),
            _ => Err(Error::invalid_type("map")),
        }
    }

    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        visitor.visit_enum(DeserializeCompound::new(self))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        if self.peek_is_null()? {
            let _ = self.next()?; //discard marker
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        let (e, _) = self.peek()?;
        match e {
            Event::Alias(_) => unimplemented!(),
            Event::Scalar(_, _, _) => match Decoder::scalar(e) {
                Ok(v) => match v {
                    Yaml::String(_) => self.deserialize_str(visitor),
                    Yaml::Integer(_) => visitor.visit_i64(self.decode_int()?),
                    Yaml::Boolean(_) => visitor.visit_bool(self.decode_bool()?),
                    Yaml::Real(_) => visitor.visit_f64(self.decode_float()?),
                    Yaml::Null => {
                        self.decode_null()?;
                        visitor.visit_none()
                    }
                    _ => unreachable!(),
                },
                Err(e) => Err(e),
            },
            Event::SequenceStart => self.deserialize_seq(visitor),
            Event::SequenceEnd => unimplemented!(),
            Event::MappingStart => self.deserialize_map(visitor),
            Event::MappingEnd => unimplemented!(),
        }
    }

    fn discard<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'toy>,
    {
        match self.next()?.0 {
            Event::MappingStart => loop {
                match self.next()?.0 {
                    Event::MappingEnd => break,
                    _ => (),
                }
            },
            Event::SequenceStart => loop {
                match self.next()?.0 {
                    Event::SequenceEnd => break,
                    _ => (),
                }
            },
            _ => (),
        };
        visitor.visit_none()
    }
}

use crate::models::table::FluxTable;
use crate::models::FieldValue;
use crate::InfluxDBError;
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserialize, Deserializer};

#[inline]
pub fn unpack<'de, T>(table: &'de mut FluxTable) -> Result<T, InfluxDBError>
where
    T: Deserialize<'de>,
{
    T::deserialize(&mut FluxTableDeserializer {
        table,
        row: 0,
        position: 0,
    })
}

pub struct FluxTableDeserializer<'a> {
    table: &'a mut FluxTable,
    row: usize,
    position: usize,
}

impl<'a> FluxTableDeserializer<'a> {
    pub fn peek(&mut self) -> Option<&FieldValue> {
        if self.remaining() {
            self.table
                .data()
                .get(self.row)
                .map(|x| x.get(self.position))
                .flatten()
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Result<&FieldValue, InfluxDBError> {
        if self.remaining() {
            let row = self.row;
            let pos = self.position;
            let fv = self.table.data().get(row).map(|x| x.get(pos)).flatten();
            self.position += 1;
            if self.table.column_size() <= self.position {
                self.position = 0;
                self.row += 1;
            }
            fv.ok_or_else(|| {
                InfluxDBError::error(format!(
                    "eof while parsing value. row:{}, position:{}",
                    row, pos
                ))
            })
        } else {
            Err(InfluxDBError::error("eof while parsing value."))
        }
    }

    pub fn remaining(&self) -> bool {
        self.table.row_size() > self.row
    }
}

fn visit<'de, V: Visitor<'de>>(visitor: V, fv: &FieldValue) -> Result<V::Value, InfluxDBError> {
    match fv {
        FieldValue::String(v) => visitor.visit_str(v),
        FieldValue::Timestamp(v) => visitor.visit_str(&v.to_rfc3339()),
        FieldValue::UInteger(v) => visitor.visit_u64(*v),
        FieldValue::Integer(v) => visitor.visit_i64(*v),
        FieldValue::Float(v) => visitor.visit_f64(*v),
        FieldValue::Boolean(v) => visitor.visit_bool(*v),
        FieldValue::Nil => visitor.visit_none(),
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut FluxTableDeserializer<'de> {
    type Error = InfluxDBError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.next().and_then(|x| visit(visitor, x))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek() {
            Some(FieldValue::Nil) => visitor.visit_none(),
            Some(_) => visitor.visit_some(self),
            None => Err(InfluxDBError::error("eof while parsing value.")),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek() {
            Some(FieldValue::Nil) => visitor.visit_none(),
            Some(_) => Err(InfluxDBError::error("invalid_type: unit.")),
            None => Err(InfluxDBError::error("eof while parsing value.")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(DeserializeAccess::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(DeserializeAccess::new(self))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(DeserializeAccess::new(self))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(DeserializeAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = self.next()?;
        visitor.visit_unit()
    }
}

pub struct DeserializeAccess<'a, 'de: 'a> {
    de: &'a mut FluxTableDeserializer<'de>,
    column_remaining: usize,
}

impl<'a, 'de> DeserializeAccess<'a, 'de> {
    pub fn new(de: &'a mut FluxTableDeserializer<'de>) -> Self {
        let column_remaining = de.table.column_size();
        Self {
            de,
            column_remaining,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for DeserializeAccess<'a, 'de> {
    type Error = InfluxDBError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.column_remaining > 0 {
            self.column_remaining -= 1;
            if self.de.remaining() {
                let h = self.de.table.header(self.de.position);
                match h {
                    Some(s) => seed.deserialize(StrDeserializer { input: s }).map(Some),
                    None => Ok(None),
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

impl<'a, 'de> SeqAccess<'de> for DeserializeAccess<'a, 'de> {
    type Error = InfluxDBError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.remaining() {
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<'a, 'de> VariantAccess<'de> for DeserializeAccess<'a, 'de> {
    type Error = InfluxDBError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        unimplemented!()
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl<'a, 'de> EnumAccess<'de> for DeserializeAccess<'a, 'de> {
    type Error = InfluxDBError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let v = seed.deserialize(&mut *self.de)?;
        Ok((v, self))
    }
}

#[derive(Clone)]
struct StrDeserializer<'a> {
    input: &'a str,
}

impl<'de, 'a> Deserializer<'de> for StrDeserializer<'a> {
    type Error = InfluxDBError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct tuple enum identifier ignored_any
    }
}

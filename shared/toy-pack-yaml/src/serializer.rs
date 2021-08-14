use crate::error::YamlError;
use crate::ser_ops::{
    SerializeArray, SerializeHash, SerializeStructVariant, SerializeTupleVariant,
};
use core::num;
use toy_pack::ser::{Serializable, Serializer};
use yaml_rust::{yaml, Yaml};

pub(crate) struct Ser;

impl Serializer for Ser {
    type Ok = Yaml;
    type Error = YamlError;
    type SeqAccessOps = SerializeArray;
    type MapAccessOps = SerializeHash;
    type StructAccessOps = SerializeHash;
    type TupleVariantOps = SerializeTupleVariant;
    type StructVariantOps = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Boolean(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Integer(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Real(match v.classify() {
            num::FpCategory::Infinite if v.is_sign_negative() => "-.inf".into(),
            num::FpCategory::Infinite => ".inf".into(),
            num::FpCategory::Nan => ".nan".into(),
            _ => {
                let mut buf = ryu::Buffer::new();
                buf.format(v).into()
            }
        }))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let vec = v.iter().map(|&b| Yaml::Integer(b as i64)).collect();
        Ok(Yaml::Array(vec))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SeqAccessOps, Self::Error> {
        Ok(SerializeArray::new(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapAccessOps, Self::Error> {
        Ok(SerializeHash::new(len))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Null)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::StructAccessOps, Self::Error> {
        Ok(SerializeHash::new(Some(len)))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::String(variant.to_string()))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        let mut hash = yaml::Hash::new();
        hash.insert(variant.serialize(Ser)?, value.serialize(Ser)?);
        Ok(Yaml::Hash(hash))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::TupleVariantOps, Self::Error> {
        Ok(SerializeTupleVariant::new(variant, len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::StructVariantOps, Self::Error> {
        Ok(SerializeStructVariant::new(variant, len))
    }

    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serializable,
    {
        v.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Null)
    }
}

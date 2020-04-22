use crate::error::YamlError;
use crate::serializer::Ser;
use toy_pack::ser::{Serializable, SerializeMapOps, SerializeSeqOps, SerializeStructOps};
use yaml_rust::{yaml, Yaml};

pub struct SerializeArray {
    array: yaml::Array,
}

pub struct SerializeHash {
    hash: yaml::Hash,
    next_key: Option<yaml::Yaml>,
}

impl SerializeArray {
    pub fn new(len: Option<usize>) -> SerializeArray {
        let array = match len {
            Some(len) => yaml::Array::with_capacity(len),
            None => yaml::Array::new(),
        };
        SerializeArray { array }
    }
}

impl SerializeHash {
    pub fn new(len: Option<usize>) -> SerializeHash {
        let hash = match len {
            Some(len) => yaml::Hash::with_capacity(len),
            None => yaml::Hash::new(),
        };
        SerializeHash {
            hash,
            next_key: None,
        }
    }
}

impl SerializeSeqOps for SerializeArray {
    type Ok = Yaml;
    type Error = YamlError;

    fn next<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.array.push(value.serialize(Ser)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Array(self.array))
    }
}

impl SerializeMapOps for SerializeHash {
    type Ok = Yaml;
    type Error = YamlError;

    fn next_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.next_key = Some(key.serialize(Ser)?);
        Ok(())
    }

    fn next_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        match self.next_key.take() {
            Some(key) => self.hash.insert(key, value.serialize(Ser)?),
            None => panic!("serialize_value called before serialize_key"),
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Hash(self.hash))
    }
}

impl SerializeStructOps for SerializeHash {
    type Ok = Yaml;
    type Error = YamlError;

    fn field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serializable,
    {
        self.hash.insert(key.serialize(Ser)?, value.serialize(Ser)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Yaml::Hash(self.hash))
    }
}

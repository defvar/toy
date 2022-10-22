use crate::error::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use toy_api::common::Format;
use toy_h::bytes::Buf;

pub fn decode<B: Buf, T: DeserializeOwned>(buf: B, format: Option<Format>) -> Result<T, Error> {
    match format.unwrap_or(Format::MessagePack) {
        Format::Json => decode_json(buf).map_err(|e| e.into()),
        Format::MessagePack => decode_mp(buf).map_err(|e| e.into()),
        Format::Yaml => decode_yaml(buf),
    }
}

pub fn encode<T>(v: &T, format: Option<Format>) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    match format.unwrap_or(Format::MessagePack) {
        Format::Json => encode_json(v),
        Format::Yaml => unimplemented!("not support"),
        Format::MessagePack => encode_mp(v),
    }
}

fn decode_json<B: Buf, T: DeserializeOwned>(mut buf: B) -> Result<T, toy_pack_json::DecodeError> {
    toy_pack_json::unpack::<T>(&buf.copy_to_bytes(buf.remaining()))
}

fn decode_mp<B: Buf, T: DeserializeOwned>(mut buf: B) -> Result<T, toy_pack_mp::DecodeError> {
    toy_pack_mp::unpack::<T>(&buf.copy_to_bytes(buf.remaining()))
}

fn decode_yaml<B: Buf, T: DeserializeOwned>(buf: B) -> Result<T, Error> {
    let s = buf_to_string(buf);
    match s {
        Ok(x) => toy_pack_yaml::unpack::<T>(x.as_str()).map_err(|x| Error::error(x)),
        Err(e) => Err(Error::error(e)),
    }
}

fn encode_json<T>(v: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    toy_pack_json::pack(v).map_err(|e| e.into())
}

fn encode_mp<T>(v: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    toy_pack_mp::pack(v).map_err(|e| e.into())
}

fn buf_to_string<T: Buf>(mut buf: T) -> Result<String, Error> {
    std::str::from_utf8(&buf.copy_to_bytes(buf.remaining()))
        .map(|x| {
            tracing::debug!("receive:{:?}", x.to_string());
            x.to_string()
        })
        .map_err(|_| Error::error("body invalid utf8 sequence."))
}

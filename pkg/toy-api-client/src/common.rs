use crate::error::ApiClientError;
use toy_api::common::Format;
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;

#[allow(dead_code)]
pub fn encode<T>(v: &T, format: Option<Format>) -> Result<Vec<u8>, ApiClientError>
where
    T: Serializable,
{
    let format = format.unwrap_or_default();
    match format {
        Format::Json => encode_json(v),
        Format::Yaml => unimplemented!("not support"),
        Format::MessagePack => encode_mp(v),
    }
}

#[allow(dead_code)]
pub fn decode<T>(bytes: &[u8], format: Option<Format>) -> Result<T, ApiClientError>
where
    T: DeserializableOwned,
{
    let format = format.unwrap_or_default();
    match format {
        Format::Json => decode_json(bytes),
        Format::Yaml => unimplemented!("not support"),
        Format::MessagePack => decode_mp(bytes),
    }
}

fn encode_json<T>(v: &T) -> Result<Vec<u8>, ApiClientError>
where
    T: Serializable,
{
    toy_pack_json::pack(v).map_err(|e| e.into())
}

fn decode_json<T>(bytes: &[u8]) -> Result<T, ApiClientError>
where
    T: DeserializableOwned,
{
    toy_pack_json::unpack(bytes).map_err(|e| e.into())
}

//////////////////////////////////////

fn encode_mp<T>(v: &T) -> Result<Vec<u8>, ApiClientError>
where
    T: Serializable,
{
    toy_pack_mp::pack(v).map_err(|e| e.into())
}

fn decode_mp<T>(bytes: &[u8]) -> Result<T, ApiClientError>
where
    T: DeserializableOwned,
{
    toy_pack_mp::unpack(bytes).map_err(|e| e.into())
}

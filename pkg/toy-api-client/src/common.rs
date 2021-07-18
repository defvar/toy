use crate::error::ApiClientError;
use toy_api::common::Format;
use toy_api::error::ErrorMessage;
use toy_h::Response;
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;

pub async fn response<T, V>(res: T, format: Option<Format>) -> Result<V, ApiClientError>
where
    T: Response,
    V: DeserializableOwned,
{
    if res.status().is_success() {
        let bytes = res.bytes().await?;
        let v = decode::<V>(&bytes, format)?;
        Ok(v)
    } else {
        let bytes = res.bytes().await?;
        let r = decode::<ErrorMessage>(&bytes, Some(Format::Json))?;
        Err(r.into())
    }
}

pub async fn no_response<T>(res: T, _format: Option<Format>) -> Result<(), ApiClientError>
where
    T: Response,
{
    if res.status().is_success() {
        let _ = res.bytes().await?;
        Ok(())
    } else {
        let bytes = res.bytes().await?;
        let r = decode::<ErrorMessage>(&bytes, Some(Format::Json))?;
        Err(r.into())
    }
}

#[allow(dead_code)]
pub fn encode<T>(v: &T, format: Option<Format>) -> Result<Vec<u8>, ApiClientError>
where
    T: Serializable,
{
    match format.unwrap_or(Format::MessagePack) {
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
    match format.unwrap_or(Format::MessagePack) {
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

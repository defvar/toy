use crate::common::error::ApiError;
use toy_api::common::Format;
use toy_h::Bytes;
use toy_pack::deser::DeserializableOwned;
use warp::{Buf, Filter};

pub fn bytes() -> impl Filter<Extract = (Bytes,), Error = warp::Rejection> + Copy {
    warp::body::bytes()
}

pub fn decode<B: Buf, T: DeserializableOwned>(
    buf: B,
    format: Option<Format>,
) -> Result<T, ApiError> {
    let format = format.unwrap_or_default();
    match format {
        Format::Json => decode_json(buf).map_err(|e| ApiError::from(e)),
        Format::MessagePack => decode_mp(buf).map_err(|e| ApiError::from(e)),
        Format::Yaml => decode_yaml(buf),
    }
}

fn decode_json<B: Buf, T: DeserializableOwned>(
    mut buf: B,
) -> Result<T, toy_pack_json::DecodeError> {
    toy_pack_json::unpack::<T>(&buf.copy_to_bytes(buf.remaining()))
}

fn decode_mp<B: Buf, T: DeserializableOwned>(mut buf: B) -> Result<T, toy_pack_mp::DecodeError> {
    toy_pack_mp::unpack::<T>(&buf.copy_to_bytes(buf.remaining()))
}

fn decode_yaml<B: Buf, T: DeserializableOwned>(buf: B) -> Result<T, ApiError> {
    let s = buf_to_string(buf);
    match s {
        Ok(x) => toy_pack_yaml::unpack::<T>(x.as_str()).map_err(|x| ApiError::error(x)),
        Err(e) => Err(ApiError::error(e)),
    }
}

fn buf_to_string<T: warp::Buf>(mut buf: T) -> Result<String, ApiError> {
    std::str::from_utf8(&buf.copy_to_bytes(buf.remaining()))
        .map(|x| {
            tracing::debug!("receive:{:?}", x.to_string());
            x.to_string()
        })
        .map_err(|_| ApiError::error("body invalid utf8 sequence."))
}

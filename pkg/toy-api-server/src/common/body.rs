use crate::common::error::ApiError;
use toy::core::prelude::Value;
use toy_pack::deser::DeserializableOwned;
use warp::{Buf, Filter};

pub fn yaml() -> impl Filter<Extract = (Value,), Error = warp::Rejection> + Clone {
    // warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::body::aggregate().and_then(|buf| async move {
        let s = buf_to_string(buf);
        match s {
            Ok(x) => toy_pack_yaml::unpack::<Value>(x.as_str())
                .map_err(|e| warp::reject::custom(ApiError::from(e))),
            Err(e) => Err(warp::reject::custom(e)),
        }
    })
}

pub fn json<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializableOwned,
{
    // warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::body::bytes().and_then(|buf| async move {
        decode_json(buf).map_err(|e| warp::reject::custom(ApiError::from(e)))
    })
}

fn decode_json<B: Buf, T: DeserializableOwned>(
    mut buf: B,
) -> Result<T, toy_pack_json::DecodeError> {
    toy_pack_json::unpack::<T>(&buf.copy_to_bytes(buf.remaining()))
}

fn buf_to_string<T: warp::Buf>(mut buf: T) -> Result<String, ApiError> {
    std::str::from_utf8(&buf.copy_to_bytes(buf.remaining()))
        .map(|x| {
            tracing::debug!("receive:{:?}", x.to_string());
            x.to_string()
        })
        .map_err(|_| ApiError::error("body invalid utf8 sequence."))
}

use crate::common::error::ApiError;
use toy_core::prelude::Value;
use warp::Filter;

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

pub fn json() -> impl Filter<Extract = (Value,), Error = warp::Rejection> + Clone {
    // warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    warp::body::aggregate().and_then(|buf| async move {
        let s = buf_to_string(buf);
        match s {
            Ok(x) => toy_pack_json::unpack::<Value>(x.as_bytes())
                .map_err(|e| warp::reject::custom(ApiError::from(e))),
            Err(e) => Err(warp::reject::custom(e)),
        }
    })
}

fn buf_to_string<T: warp::Buf>(buf: T) -> Result<String, ApiError> {
    std::str::from_utf8(buf.bytes())
        .map(|x| {
            tracing::debug!("receive:{:?}", x.to_string());
            x.to_string()
        })
        .map_err(|_| ApiError::error("body invalid utf8 sequence."))
}

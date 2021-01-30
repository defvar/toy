use futures_util::stream::Stream;
use toy_api::common::Format;
use toy_pack::ser::Serializable;
use toy_pack_json::EncodeError;
use warp::http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE, TRANSFER_ENCODING};
use warp::http::StatusCode;
use warp::reply::Reply;
use warp::reply::Response;

pub fn into_response<T>(v: &T, format: Option<Format>) -> Response
where
    T: Serializable,
{
    let format = format.unwrap_or(Format::default());
    match format {
        Format::Json => json(v).into_response(),
        Format::Yaml => yaml(v).into_response(),
        Format::MessagePack => mp(v).into_response(),
    }
}

pub fn mp<T>(v: &T) -> Mp
where
    T: Serializable,
{
    Mp {
        inner: toy_pack_mp::pack(v)
            .map_err(|e| tracing::error!("reply::message pack error: {}", e)),
    }
}

pub fn yaml<T>(v: &T) -> Yaml
where
    T: Serializable,
{
    Yaml {
        inner: toy_pack_yaml::pack_to_string(v)
            .map_err(|e| tracing::error!("reply::yaml error: {}", e)),
    }
}

pub fn json<T>(v: &T) -> Json
where
    T: Serializable,
{
    Json {
        inner: toy_pack_json::pack_to_string(v)
            .map_err(|e| tracing::error!("reply::json error: {}", e)),
    }
}

pub fn json_stream<St>(stream: St) -> JsonStream<St>
where
    St: Stream<Item = Result<String, EncodeError>> + Send + 'static,
{
    JsonStream { inner: stream }
}

pub struct Mp {
    inner: Result<Vec<u8>, ()>,
}

impl warp::Reply for Mp {
    fn into_response(self) -> Response {
        match self.inner {
            Ok(body) => {
                let mut res = Response::new(body.into());
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x-msgpack"),
                );
                res
            }
            Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub struct Yaml {
    inner: Result<String, ()>,
}

impl warp::Reply for Yaml {
    #[inline]
    fn into_response(self) -> Response {
        match self.inner {
            Ok(body) => {
                let mut res = Response::new(body.into());
                res.headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static("application/yaml"));
                res
            }
            Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub struct Json {
    inner: Result<String, ()>,
}

impl warp::Reply for Json {
    #[inline]
    fn into_response(self) -> Response {
        match self.inner {
            Ok(body) => {
                let mut res = Response::new(body.into());
                res.headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                res
            }
            Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub struct JsonStream<St> {
    inner: St,
}

impl<St> warp::Reply for JsonStream<St>
where
    St: Stream<Item = Result<String, EncodeError>> + Send + 'static,
{
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::new(warp::hyper::Body::wrap_stream(self.inner));
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        res.headers_mut()
            .insert(TRANSFER_ENCODING, HeaderValue::from_static("chunked"));
        res
    }
}

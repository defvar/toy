use crate::ApiError;
use futures_util::stream::Stream;
use serde::Serialize;
use std::marker::PhantomData;
use toy_api::common::Format;
use warp::http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE, TRANSFER_ENCODING};
use warp::http::StatusCode;
use warp::hyper::body::Bytes;
use warp::reply::Reply;
use warp::reply::Response;

pub fn into_response<T>(v: &T, format: Option<Format>, pretty: Option<bool>) -> Response
where
    T: Serialize,
{
    let format = format.unwrap_or(Format::default());
    match format {
        Format::Json => json(v, pretty).into_response(),
        Format::Yaml => yaml(v).into_response(),
        Format::MessagePack => mp(v).into_response(),
    }
}

pub fn into_response_stream<St, B>(stream: St, format: Option<Format>) -> ReplyStream<St, B>
where
    St: Stream<Item = Result<B, ApiError>> + Send + 'static,
    B: Into<Bytes> + Send + 'static,
{
    let format = format.unwrap_or(Format::default());
    ReplyStream {
        inner: stream,
        content_type: ResponseContentType::from_format(format),
        t: PhantomData,
    }
}

pub fn encode<T>(v: &T, format: Option<Format>, pretty: Option<bool>) -> Result<Bytes, ApiError>
where
    T: Serialize,
{
    let format = format.unwrap_or_default();
    match format {
        Format::Json if pretty.is_some() && pretty.unwrap() => toy_pack_json::pack_pretty(v)
            .map(Bytes::from)
            .map_err(|e| ApiError::error(e)),
        Format::Json => toy_pack_json::pack(v)
            .map(Bytes::from)
            .map_err(|e| ApiError::error(e)),
        Format::Yaml => toy_pack_yaml::pack_to_string(v)
            .map(Bytes::from)
            .map_err(|e| ApiError::error(e)),
        Format::MessagePack => toy_pack_mp::pack(v)
            .map(Bytes::from)
            .map_err(|e| ApiError::error(e)),
    }
}

fn mp<T>(v: &T) -> Mp
where
    T: Serialize,
{
    Mp {
        inner: toy_pack_mp::pack(v)
            .map_err(|e| tracing::error!("reply::message pack error: {:?}", e)),
    }
}

fn yaml<T>(v: &T) -> Yaml
where
    T: Serialize,
{
    Yaml {
        inner: toy_pack_yaml::pack_to_string(v)
            .map_err(|e| tracing::error!("reply::yaml error: {:?}", e)),
    }
}

fn json<T>(v: &T, pretty: Option<bool>) -> Json
where
    T: Serialize,
{
    if pretty.is_some() && pretty.unwrap() {
        Json {
            inner: toy_pack_json::pack_to_string_pretty(v)
                .map_err(|e| tracing::error!("reply::json error: {:?}", e)),
        }
    } else {
        Json {
            inner: toy_pack_json::pack_to_string(v)
                .map_err(|e| tracing::error!("reply::json error: {:?}", e)),
        }
    }
}

pub enum ResponseContentType {
    Json,
    MessagePack,
    Yaml,
}

impl ResponseContentType {
    pub fn to_header_value(&self) -> HeaderValue {
        match self {
            ResponseContentType::Json => HeaderValue::from_static("application/json"),
            ResponseContentType::MessagePack => HeaderValue::from_static("application/x-msgpack"),
            ResponseContentType::Yaml => HeaderValue::from_static("application/yaml"),
        }
    }

    pub fn from_format(v: Format) -> ResponseContentType {
        match v {
            Format::Json => ResponseContentType::Json,
            Format::MessagePack => ResponseContentType::MessagePack,
            Format::Yaml => ResponseContentType::Yaml,
        }
    }
}

struct Mp {
    inner: Result<Vec<u8>, ()>,
}

impl warp::Reply for Mp {
    fn into_response(self) -> Response {
        match self.inner {
            Ok(body) => {
                let mut res = Response::new(body.into());
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    ResponseContentType::MessagePack.to_header_value(),
                );
                res
            }
            Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

struct Yaml {
    inner: Result<String, ()>,
}

impl warp::Reply for Yaml {
    #[inline]
    fn into_response(self) -> Response {
        match self.inner {
            Ok(body) => {
                let mut res = Response::new(body.into());
                res.headers_mut()
                    .insert(CONTENT_TYPE, ResponseContentType::Yaml.to_header_value());
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
                    .insert(CONTENT_TYPE, ResponseContentType::Json.to_header_value());
                res
            }
            Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub struct ReplyStream<St, B> {
    inner: St,
    content_type: ResponseContentType,
    t: PhantomData<B>,
}

impl<St, B> warp::Reply for ReplyStream<St, B>
where
    St: Stream<Item = Result<B, ApiError>> + Send + 'static,
    B: Into<Bytes> + Send + 'static,
{
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::new(warp::hyper::Body::wrap_stream(self.inner));
        res.headers_mut()
            .insert(CONTENT_TYPE, self.content_type.to_header_value());
        res.headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        res.headers_mut()
            .insert(TRANSFER_ENCODING, HeaderValue::from_static("chunked"));
        res
    }
}

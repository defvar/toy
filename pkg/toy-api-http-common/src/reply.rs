use crate::error::Error;
use axum::response::IntoResponse;
use bytes::Bytes;
use futures_util::stream::Stream;
use http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE, TRANSFER_ENCODING};
use http::StatusCode;
use serde::Serialize;
use std::marker::PhantomData;
use toy_api::common::{Format, Indent, ListObject};
use toy_api::selection::fields::Fields;

pub fn into_response_with_fields<T>(
    v: &T,
    format: Option<Format>,
    indent: Option<Indent>,
    fields: &Fields,
) -> Result<axum::response::Response, Error>
where
    T: Serialize,
{
    if fields.is_empty() {
        Ok(into_response(v, format, indent))
    } else {
        let value = toy_core::data::pack(v)?;
        let applied_value = fields.apply(&value).map_err(|e| Error::invalid_field(e))?;
        Ok(into_response(&applied_value, format, indent))
    }
}

pub fn into_list_item_response_with_fields<T, V>(
    list: &T,
    format: Option<Format>,
    indent: Option<Indent>,
    fields: &Fields,
) -> Result<axum::response::Response, Error>
where
    T: Serialize + ListObject<V>,
    V: Serialize,
{
    if fields.is_empty() {
        Ok(into_response(list, format, indent))
    } else {
        let applied_list = list.items().iter().try_fold(
            Vec::with_capacity(list.count() as usize),
            |mut acc, item| {
                let value = toy_core::data::pack(item)?;
                match fields.apply(&value) {
                    Ok(applied_value) => {
                        acc.push(applied_value);
                        Ok(acc)
                    }
                    Err(f) => Err(Error::invalid_field(f)),
                }
            },
        )?;
        Ok(into_response(&applied_list, format, indent))
    }
}

pub fn into_response<T>(
    v: &T,
    format: Option<Format>,
    indent: Option<Indent>,
) -> axum::response::Response
where
    T: Serialize,
{
    let format = format.unwrap_or(Format::default());
    match format {
        Format::Json => IntoResponse::into_response(json(v, indent)),
        Format::Yaml => IntoResponse::into_response(yaml(v)),
        Format::MessagePack => IntoResponse::into_response(mp(v)),
    }
}

pub fn into_response_stream<St, B>(stream: St, format: Option<Format>) -> ReplyStream<St, B>
where
    St: Stream<Item = Result<B, Error>> + Send + 'static,
    B: Into<Bytes> + Send + 'static,
{
    let format = format.unwrap_or(Format::default());
    ReplyStream {
        inner: stream,
        content_type: ResponseContentType::from_format(format),
        t: PhantomData,
    }
}

pub fn encode<T>(v: &T, format: Option<Format>, pretty: Option<bool>) -> Result<Bytes, Error>
where
    T: Serialize,
{
    let format = format.unwrap_or_default();
    match format {
        Format::Json if pretty.is_some() && pretty.unwrap() => toy_pack_json::pack_pretty(v)
            .map(Bytes::from)
            .map_err(|e| Error::error(e)),
        Format::Json => toy_pack_json::pack(v)
            .map(Bytes::from)
            .map_err(|e| Error::error(e)),
        Format::Yaml => toy_pack_yaml::pack_to_string(v)
            .map(Bytes::from)
            .map_err(|e| Error::error(e)),
        Format::MessagePack => toy_pack_mp::pack(v)
            .map(Bytes::from)
            .map_err(|e| Error::error(e)),
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

fn json<T>(v: &T, indent: Option<Indent>) -> Json
where
    T: Serialize,
{
    if indent.is_some() && indent.unwrap() == Indent::Pretty {
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

impl IntoResponse for Mp {
    fn into_response(self) -> axum::response::Response {
        match self.inner {
            Ok(body) => {
                let mut res = IntoResponse::into_response(body);
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    ResponseContentType::MessagePack.to_header_value(),
                );
                res
            }
            Err(()) => IntoResponse::into_response(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

struct Yaml {
    inner: Result<String, ()>,
}

impl IntoResponse for Yaml {
    #[inline]
    fn into_response(self) -> axum::response::Response {
        match self.inner {
            Ok(body) => {
                let mut res = IntoResponse::into_response(body);
                res.headers_mut()
                    .insert(CONTENT_TYPE, ResponseContentType::Yaml.to_header_value());
                res
            }
            Err(()) => IntoResponse::into_response(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct Json {
    inner: Result<String, ()>,
}

impl IntoResponse for Json {
    fn into_response(self) -> axum::response::Response {
        match self.inner {
            Ok(body) => {
                let mut res = IntoResponse::into_response(body);
                res.headers_mut()
                    .insert(CONTENT_TYPE, ResponseContentType::Json.to_header_value());
                res
            }
            Err(()) => IntoResponse::into_response(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct ReplyStream<St, B> {
    inner: St,
    content_type: ResponseContentType,
    t: PhantomData<B>,
}

impl<St, B> IntoResponse for ReplyStream<St, B>
where
    St: Stream<Item = Result<B, Error>> + Send + 'static,
    B: Into<Bytes> + Send + 'static,
{
    #[inline]
    fn into_response(self) -> axum::response::Response {
        let mut res = IntoResponse::into_response(axum::body::StreamBody::new(self.inner));
        res.headers_mut()
            .insert(CONTENT_TYPE, self.content_type.to_header_value());
        res.headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        res.headers_mut()
            .insert(TRANSFER_ENCODING, HeaderValue::from_static("chunked"));
        res
    }
}

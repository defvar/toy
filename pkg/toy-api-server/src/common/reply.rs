use toy_pack::ser::Serializable;
use warp::http::header::{HeaderValue, CONTENT_TYPE};
use warp::http::StatusCode;
use warp::reply::Response;

pub fn yaml<T>(v: &T) -> Yaml
where
    T: Serializable,
{
    Yaml {
        inner: toy_pack_yaml::pack_to_string(v)
            .map_err(|e| log::error!("reply::yaml error: {}", e)),
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

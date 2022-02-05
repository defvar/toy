use warp::http::header::{AUTHORIZATION, CONTENT_TYPE};
use warp::http::HeaderValue;
use warp::test::RequestBuilder;

pub fn prepare() {
    std::env::set_var("TOY_AUTHORIZATION", "none");
}

pub fn get() -> RequestBuilder {
    warp::test::request()
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(AUTHORIZATION, HeaderValue::from_static("Bearer dummy"))
        .method("GET")
}

pub fn put() -> RequestBuilder {
    warp::test::request()
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(AUTHORIZATION, HeaderValue::from_static("Bearer dummy"))
        .method("PUT")
}

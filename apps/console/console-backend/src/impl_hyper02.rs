use async_trait::async_trait;
use std::convert::TryFrom;
use std::sync::Arc;
use toy_api_server::toy_h::error::HError;
use toy_api_server::toy_h::{Bytes, HttpClient, RequestBuilder, Response};
use toy_api_server::warp::hyper::client::HttpConnector;
use toy_api_server::warp::hyper::header::{HeaderName, HeaderValue};
use toy_api_server::warp::hyper::http::request::Builder;
use toy_api_server::warp::hyper::{
    body, Body, Client, HeaderMap, Method, Request, Response as HyperResponse, StatusCode, Uri,
    Version,
};

#[derive(Clone)]
pub struct Hyper02Client {
    client: Arc<Client<HttpConnector, Body>>,
}

pub struct Hyper02Builder {
    raw: Builder,
    client: Arc<Client<HttpConnector, Body>>,
    body: Option<Bytes>,
}

pub struct Hyper02Response {
    raw: HyperResponse<Body>,
}

impl Hyper02Client {
    pub fn from(client: Arc<Client<HttpConnector, Body>>) -> Hyper02Client {
        Hyper02Client { client }
    }
}

impl HttpClient for Hyper02Client {
    type Builder = Hyper02Builder;

    fn get<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        self.request(Method::GET, uri)
    }

    fn post<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        self.request(Method::POST, uri)
    }

    fn put<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        self.request(Method::PUT, uri)
    }

    fn patch<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        self.request(Method::PATCH, uri)
    }

    fn delete<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        self.request(Method::DELETE, uri)
    }

    fn head<T: Into<Uri>>(&self, uri: T) -> Self::Builder {
        self.request(Method::HEAD, uri)
    }

    fn request<T: Into<Uri>>(&self, method: Method, uri: T) -> Self::Builder {
        let raw = Request::builder().method(method).uri(uri.into());
        Hyper02Builder {
            raw,
            client: Arc::clone(&self.client),
            body: None,
        }
    }
}

#[async_trait]
impl RequestBuilder for Hyper02Builder {
    type Builder = Hyper02Builder;
    type Response = Hyper02Response;

    fn header<K, V>(self, key: K, value: V) -> Self::Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<toy_api_server::toy_h::http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<toy_api_server::toy_h::http::Error>,
    {
        let raw = self.raw.header(key, value);
        Hyper02Builder {
            raw,
            client: Arc::clone(&self.client),
            body: None,
        }
    }

    fn headers(self, _headers: HeaderMap<HeaderValue>) -> Self::Builder {
        unimplemented!()
    }

    fn body<T: Into<Bytes>>(self, body: T) -> Self::Builder {
        let client = Arc::clone(&self.client);
        Hyper02Builder {
            raw: self.raw,
            client,
            body: Some(body.into()),
        }
    }

    async fn send(mut self) -> Result<Self::Response, HError> {
        // Bytes version convert....
        let body = self.body.take().unwrap().to_vec();
        let req = self.raw.body(Body::from(body)).unwrap();

        self.client
            .request(req)
            .await
            .map(|x| Hyper02Response { raw: x })
            .map_err(|e| HError::error(e))
    }
}

#[async_trait]
impl Response for Hyper02Response {
    fn status(&self) -> StatusCode {
        self.raw.status()
    }

    fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.raw.headers()
    }

    async fn bytes(self) -> Result<Bytes, HError> {
        // Bytes version convert....
        let b_warp = body::to_bytes(self.raw).await.unwrap();
        Ok(Bytes::from(b_warp.to_vec()))
    }

    fn version(&self) -> Version {
        self.raw.version()
    }
}

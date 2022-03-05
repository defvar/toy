use crate::auth::Auth;
use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use toy_api::common::{
    DeleteOption, FindOption, Format, ListOption, ListOptionLike, PostOption, PutOption,
};
use toy_api::error::ErrorMessage;
use toy_h::{HeaderMap, HttpClient, RequestBuilder, Response, Uri};
use toy_pack_urlencoded::QueryParseError;

pub async fn find<T, V>(
    client: &T,
    auth: Option<&Auth>,
    root: &str,
    path: &str,
    key: &str,
    opt: FindOption,
) -> Result<V, Error>
where
    T: HttpClient,
    V: DeserializeOwned,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}/{}?{}", root, path, key, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let r = client.get(uri).headers(h).send().await?;
    response(r, opt.format()).await
}

pub async fn list<T, V>(
    client: &T,
    auth: Option<&Auth>,
    root: &str,
    path: &str,
    opt: ListOption,
) -> Result<V, Error>
where
    T: HttpClient,
    V: DeserializeOwned,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}?{}", root, path, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let r = client.get(uri).headers(h).send().await?;
    response(r, opt.format()).await
}

pub async fn list_with_opt<T, V, O>(
    client: &T,
    auth: Option<&Auth>,
    root: &str,
    path: &str,
    opt: O,
) -> Result<V, Error>
where
    T: HttpClient,
    V: DeserializeOwned,
    O: Serialize + ListOptionLike,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}?{}", root, path, query).parse::<Uri>()?;
    let h = common_headers(opt.common().format(), auth);
    let r = client.get(uri).headers(h).send().await?;
    response(r, opt.common().format()).await
}

pub async fn put<T, V>(
    client: &T,
    auth: Option<&Auth>,
    root: &str,
    path: &str,
    key: &str,
    v: &V,
    opt: PutOption,
) -> Result<(), Error>
where
    T: HttpClient,
    V: Serialize,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}/{}?{}", root, path, key, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let body = crate::codec::encode(&v, opt.format())?;
    let r = client.put(uri).headers(h).body(body).send().await?;
    response(r, opt.format()).await
}

pub async fn post<T, V, R>(
    client: &T,
    auth: Option<&Auth>,
    root: &str,
    path: &str,
    v: &V,
    opt: PostOption,
) -> Result<R, Error>
where
    T: HttpClient,
    V: Serialize,
    R: DeserializeOwned,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}?{}", root, path, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let body = crate::codec::encode(&v, opt.format())?;
    let r = client.post(uri).headers(h).body(body).send().await?;
    response(r, opt.format()).await
}

pub async fn delete<T>(
    client: &T,
    auth: &Auth,
    root: &str,
    path: &str,
    key: &str,
    opt: DeleteOption,
) -> Result<(), Error>
where
    T: HttpClient,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}/{}?{}", root, path, key, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), Some(auth));
    let r = client.delete(uri).headers(h).send().await?;
    response(r, opt.format()).await
}

pub fn common_headers(format: Option<Format>, auth: Option<&Auth>) -> toy_h::HeaderMap {
    use toy_h::{header::HeaderValue, header::AUTHORIZATION, header::CONTENT_TYPE};

    let mut headers = HeaderMap::new();

    headers.insert("X-Toy-Api-Client", HeaderValue::from_static("toy-rs"));

    let v = match format.unwrap_or(Format::MessagePack) {
        Format::Json => HeaderValue::from_static("application/json"),
        Format::MessagePack => HeaderValue::from_static("application/x-msgpack"),
        Format::Yaml => HeaderValue::from_static("application/yaml"),
    };
    headers.insert(CONTENT_TYPE, v);

    if let Some(auth) = auth {
        if let Some(token) = auth.bearer_token() {
            match HeaderValue::from_str(&format!("Bearer {}", token)) {
                Ok(h) => {
                    headers.insert(AUTHORIZATION, h);
                }
                Err(_) => {}
            }
        }
    }

    headers
}

async fn response<T, V>(res: T, format: Option<Format>) -> Result<V, Error>
where
    T: Response,
    V: DeserializeOwned,
{
    if res.status().is_success() {
        let bytes = res.bytes().await?;
        crate::codec::decode::<_, V>(bytes, format)
    } else {
        let bytes = res.bytes().await?;
        let e = crate::codec::decode::<_, ErrorMessage>(bytes, Some(Format::Json))?;
        Err(e.into())
    }
}

pub(crate) fn prepare_query<T>(p: &T) -> Result<String, QueryParseError>
where
    T: Serialize,
{
    #[derive(Serialize)]
    struct DefaultFormat {
        format: Format,
    }

    let mut q: String = toy_pack_urlencoded::pack_to_string(p)?;
    if !q.contains("format") {
        if q.contains("=") {
            q.push('&');
        }
        let q2 = toy_pack_urlencoded::pack_to_string(&DefaultFormat {
            format: Format::MessagePack,
        })?;
        q.push_str(&q2);
    }
    Ok(q)
}

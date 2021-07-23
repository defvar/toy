use crate::auth::Auth;
use crate::error::ApiClientError;
use toy_api::common::{DeleteOption, FindOption, Format, ListOption, PutOption};
use toy_h::{HeaderMap, HttpClient, RequestBuilder, Uri};
use toy_pack::deser::DeserializableOwned;
use toy_pack::ser::Serializable;
use toy_pack::Pack;
use toy_pack_urlencoded::QueryParseError;

pub(crate) async fn find<T, V>(
    client: &T,
    auth: &Auth,
    root: &str,
    path: &str,
    key: &str,
    opt: FindOption,
) -> Result<V, ApiClientError>
where
    T: HttpClient,
    V: DeserializableOwned,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}/{}?{}", root, path, key, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let r = client.get(uri).headers(h).send().await?;
    crate::common::response(r, opt.format()).await
}

pub(crate) async fn list<T, V>(
    client: &T,
    auth: &Auth,
    root: &str,
    path: &str,
    opt: ListOption,
) -> Result<V, ApiClientError>
where
    T: HttpClient,
    V: DeserializableOwned,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}?{}", root, path, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let r = client.get(uri).headers(h).send().await?;
    crate::common::response(r, opt.format()).await
}

pub(crate) async fn put<T, V>(
    client: &T,
    auth: &Auth,
    root: &str,
    path: &str,
    key: &str,
    v: &V,
    opt: PutOption,
) -> Result<(), ApiClientError>
where
    T: HttpClient,
    V: Serializable,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}/{}?{}", root, path, key, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let body = crate::common::encode(&v, opt.format())?;
    let r = client.put(uri).headers(h).body(body).send().await?;
    crate::common::no_response(r, opt.format()).await
}

pub(crate) async fn delete<T>(
    client: &T,
    auth: &Auth,
    root: &str,
    path: &str,
    key: &str,
    opt: DeleteOption,
) -> Result<(), ApiClientError>
where
    T: HttpClient,
{
    let query = prepare_query(&opt)?;
    let uri = format!("{}/{}/{}?{}", root, path, key, query).parse::<Uri>()?;
    let h = common_headers(opt.format(), auth);
    let r = client.delete(uri).headers(h).send().await?;
    crate::common::no_response(r, opt.format()).await
}

pub(crate) fn common_headers(format: Option<Format>, auth: &Auth) -> toy_h::HeaderMap {
    use toy_h::{header::HeaderValue, header::AUTHORIZATION, header::CONTENT_TYPE};

    let mut headers = HeaderMap::new();

    headers.insert("X-Toy-Api-Client", HeaderValue::from_static("toy-rs"));

    let v = match format.unwrap_or(Format::MessagePack) {
        Format::Json => HeaderValue::from_static("application/json"),
        Format::MessagePack => HeaderValue::from_static("application/x-msgpack"),
        Format::Yaml => HeaderValue::from_static("application/yaml"),
    };
    headers.insert(CONTENT_TYPE, v);

    if auth.bearer_token().is_some() {
        match HeaderValue::from_str(&format!("Bearer {}", auth.bearer_token().unwrap())) {
            Ok(h) => {
                headers.insert(AUTHORIZATION, h);
            }
            Err(_) => {}
        }
    }

    headers
}

pub(crate) fn prepare_query<T>(p: &T) -> Result<String, QueryParseError>
where
    T: Serializable,
{
    #[derive(Pack)]
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

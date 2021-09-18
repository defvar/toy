use crate::ApiError;
use serde::de::DeserializeOwned;
use warp::Filter;

pub fn query_opt<T>() -> impl Filter<Extract = (Option<T>,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    query::<T>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<T>,), warp::Rejection>((None,)) })
}

pub fn query<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    warp::query::raw().and_then(|h: String| {
        tracing::debug!("query:{:?}", h);
        let r = toy_pack_urlencoded::unpack::<T>(h.as_bytes())
            .map_err(|e| ApiError::query_parse(e).into_rejection());
        std::future::ready(r)
    })
}

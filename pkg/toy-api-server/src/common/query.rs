use serde::de::DeserializeOwned;
use warp::Filter;

pub fn query_opt<T>() -> impl Filter<Extract = (Option<T>,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send + 'static,
{
    query::<T>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<T>,), warp::Rejection>((None,)) })
}

pub fn query<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send + 'static,
{
    warp::query::<T>()
}

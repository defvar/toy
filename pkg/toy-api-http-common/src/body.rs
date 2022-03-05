use toy_h::bytes::Bytes;
use warp::Filter;

pub fn bytes() -> impl Filter<Extract = (Bytes,), Error = warp::Rejection> + Copy {
    warp::body::bytes()
}

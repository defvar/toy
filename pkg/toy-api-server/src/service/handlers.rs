use crate::service::ServiceRegistry;
use std::convert::Infallible;

pub async fn list<S, F>(reg: ServiceRegistry<S, F>) -> Result<impl warp::Reply, Infallible>
where
    S: Send,
    F: Sync,
{
    Ok(format!("{:?}", reg))
}

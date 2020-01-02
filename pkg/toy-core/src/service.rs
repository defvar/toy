use super::context::Context;
use super::data::Frame;
use super::data::FrameFuture;
use super::error::Error;
use super::error::MessagingError;
use futures::Future;

pub trait Service:
    Handler<
        Request = Frame,
        Response = Frame,
        Error = MessagingError,
        Future = FrameFuture<MessagingError>,
    > + Send
    + Sync
{
    fn start(&mut self) {}

    fn completed(&mut self) {}
}

pub trait Handler {
    type Request;
    type Response;
    type Error: Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn handle(&self, ctx: &Context, req: Self::Request) -> Self::Future;
}

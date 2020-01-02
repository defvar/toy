use futures::executor::block_on;
use log::info;
use std::thread;
use toy_core::context::Context;
use toy_core::data::{Frame, FrameFuture, Value};
use toy_core::error::MessagingError;
use toy_core::flow::Flow;
use toy_core::registry::Registry;
use toy_core::service::{Handler, Service};

struct Hello;

impl Service for Hello {}

impl Handler for Hello {
    type Request = Frame;
    type Response = Frame;
    type Error = MessagingError;
    type Future = FrameFuture<Self::Error>;

    fn handle(&self, _ctx: &Context, _arg: Self::Request) -> Self::Future {
        thread::sleep(std::time::Duration::from_secs(1));
        Frame::from("hello".to_string())
            .into_end_frame()
            .into_future()
    }
}

struct Bye;

impl Service for Bye {}

impl Handler for Bye {
    type Request = Frame;
    type Response = Frame;
    type Error = MessagingError;
    type Future = FrameFuture<Self::Error>;

    fn handle(&self, _ctx: &Context, arg: Self::Request) -> Self::Future {
        thread::sleep(std::time::Duration::from_secs(1));
        (match arg.get_value() {
            Value::String(v) => Frame::from(format!("{}{}", v, "-bye")),
            _ => Frame::none(),
        })
        .into_end_frame()
        .into_future()
    }
}

struct End;

impl Service for End {}

impl Handler for End {
    type Request = Frame;
    type Response = Frame;
    type Error = MessagingError;
    type Future = FrameFuture<Self::Error>;

    fn handle(&self, _ctx: &Context, _arg: Self::Request) -> Self::Future {
        thread::sleep(std::time::Duration::from_secs(1));
        Frame::none().into_end_frame().into_future()
    }
}

fn as_raw_bytes<T: ?Sized>(x: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(x as *const T as *const u8, std::mem::size_of_val(x)) }
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    info!("hello example");

    Registry::set("myHello", Hello);
    Registry::set("myBye", Bye);
    Registry::set("myEnd", End);

    info!("-----------------------------------");
    info!("main thread {:?}", thread::current().id());

    let commands = vec![
        "myHello".to_string(),
        "myBye".to_string(),
        "myEnd".to_string(),
    ];

    let ok = block_on(Flow::new(commands).start());
    info!("{:?}", ok);
}

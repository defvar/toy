use futures::channel::mpsc::{Receiver, Sender};
use futures::executor::{block_on, ThreadPool};
use futures::{SinkExt, StreamExt};
use log::{error, info};
use std::sync::Arc;
use std::thread;
use toy_core::channel;
use toy_core::context::Context;
use toy_core::data::{Frame, FrameFuture, Value};
use toy_core::error::MessagingError;
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
        Frame::from("hello".to_string()).future()
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
        (match arg.get_value() {
            Value::String(v) => Frame::from(format!("{}{}", v, "-bye")),
            _ => Frame::none(),
        })
        .future()
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
        Frame::none().future()
    }
}

fn as_raw_bytes<T: ?Sized>(x: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(x as *const T as *const u8, std::mem::size_of_val(x)) }
}

async fn process0(
    mut tx: Sender<Result<Frame, MessagingError>>,
    mut rx: Receiver<Result<Frame, MessagingError>>,
    commands: Arc<Vec<String>>,
    count: usize,
) -> Result<(), MessagingError> {
    //prepare handler and context
    let h = Registry::get(commands.get(count).unwrap()).unwrap();
    let c = Context;
    let name = commands.get(count).unwrap();

    info!("{:?} start serivce {:?}", thread::current().id(), name);

    //main loop, receive on message
    while let Some(item) = rx.next().await {
        match item {
            Ok(r) => {
                info!(
                    "{:?} serivce:{:?} receive {:?}",
                    thread::current().id(),
                    name,
                    r
                );
                let r = h.handle(&c, r).await;
                info!(
                    "{:?} serivce:{:?} send value {:?}",
                    thread::current().id(),
                    name,
                    r
                );
                let _ = tx.send(r).await?;
                thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => {
                error!("error {:?}", e);
                let _ = tx.send(Err(MessagingError::error(e))).await?;
            }
        };
        break;
    }
    info!("{:?} end serivce:{:?}", thread::current().id(), name);

    Ok(())
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("hello example");

    Registry::set("myHello", Hello);
    Registry::set("myBye", Bye);
    Registry::set("myEnd", End);

    info!("-----------------------------------");
    info!("main thread {:?}", thread::current().id());

    let pool = ThreadPool::new().expect("Failed to build pool");

    let commands = Arc::new(vec![
        "myHello".to_string(),
        "myBye".to_string(),
        "myEnd".to_string(),
    ]);

    let (mut tx0, rx1) = channel::stream::<Result<Frame, MessagingError>>(12);
    let (tx1, rx2) = channel::stream::<Result<Frame, MessagingError>>(12);
    let (tx2, rx3) = channel::stream::<Result<Frame, MessagingError>>(12);
    let (tx3, rx0) = channel::stream::<Result<Frame, MessagingError>>(12);

    let c1 = commands.clone();
    let c2 = commands.clone();
    let c3 = commands.clone();

    pool.spawn_ok(async {
        if let Err(e) = process0(tx1, rx1, c1, 0).await {
            error!("an error occured; error = {:?}", e);
        }
    });
    pool.spawn_ok(async {
        if let Err(e) = process0(tx2, rx2, c2, 1).await {
            error!("an error occured; error = {:?}", e);
        }
    });
    pool.spawn_ok(async {
        if let Err(e) = process0(tx3, rx3, c3, 2).await {
            error!("an error occured; error = {:?}", e);
        }
    });

    // start
    let _ = block_on(tx0.send(Ok(Frame::none())));

    let result: Vec<Result<Frame, MessagingError>> = block_on({
        rx0.map(|x| match x {
            Ok(r) => {
                info!("all end thread {:?}", thread::current().id());
                Ok(r)
            }
            Err(e) => {
                error!("error {:?}", e);
                Err(e)
            }
        })
        .collect()
    });
    info!("end {:?}", result);
}

use futures::executor::block_on;
use futures::future::ok;
use log::info;
use std::any::Any;
use std::thread;
use toy_core::context::{Context, ContextFactory};
use toy_core::context_box;
use toy_core::data::Frame;
use toy_core::error::MessagingError;
use toy_core::service::{self, Handler2};
use toy_core::service_box::{self, BoxServiceFactory};

fn as_raw_bytes<T: ?Sized>(x: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(x as *const T as *const u8, std::mem::size_of_val(x)) }
}

pub async fn aiueo(_ctx: &mut ServiceContext2, _req: Frame) -> Result<(), MessagingError> {
    println!("aiueo!");
    Ok(())
}

pub async fn kakiku(_ctx: &mut ServiceContext3, _req: Frame) -> Result<(), MessagingError> {
    println!("kakiku!");
    Ok(())
}

#[derive(Clone, Debug)]
pub struct ServiceContext2 {
    id: String,
}

pub struct ServiceContext3;

impl Context for ServiceContext2 {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Context for ServiceContext3 {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct ServiceContext2Factory;

impl ContextFactory for ServiceContext2Factory {
    type Context = ServiceContext2;

    fn new_context(&self) -> Self::Context {
        ServiceContext2 { id: "1".to_owned() }
    }
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp_nanos();
    builder.init();

    info!("-----------------------------------");
    info!("main thread {:?}", thread::current().id());

    let _commands = vec![
        "myHello".to_string(),
        "myBye".to_string(),
        "myEnd".to_string(),
    ];

    //let ok = block_on(Flow::new(commands).start());
    //let factory = fn_service_factory(|| ok::<_, ()>(fn_service(aiueo)));
    //    let mut s = fn_service(aiueo);
    let factory = service::fn_service_factory(|| {
        ok::<_, ()>(service::fn_service(
            |_ctx: &mut ServiceContext2, _req: Frame| {
                async {
                    println!("aiueo!");
                    Result::<(), MessagingError>::Ok(())
                }
            },
        ))
    });

    let mut vec: Vec<BoxServiceFactory<Frame, MessagingError, ()>> = Vec::new();
    vec.push(service_box::boxed(factory));

    let ctx_factory = context_box::boxed(ServiceContext2Factory);
    let mut box_ctx = ctx_factory.new_context();

    let mut service = block_on(vec.get(0).unwrap().new_handler()).unwrap();
    //    let mut service2 = block_on(factory.new_handler()).unwrap();
    let ok = block_on(service.handle(&mut box_ctx, Frame::none()));
    info!("{:?}", ok);
}

use crate::codec;
use crate::config::FunctionConfig;
use std::cell::{OnceCell, UnsafeCell};
use std::future::Future;
use std::sync::{Once};
use toy_core::prelude::*;
use v8::{OwnedIsolate};

#[derive(Clone, Debug)]
pub struct Function;

pub struct FunctionContext {
    config: FunctionConfig,
}

impl Service for Function {
    type Context = FunctionContext;
    type Request = Frame;
    type Future =
    impl Future<Output=Result<ServiceContext<FunctionContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
    impl Future<Output=Result<ServiceContext<FunctionContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
    impl Future<Output=Result<ServiceContext<FunctionContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::flow()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        ctx: Self::Context,
        req: Self::Request,
        tx: Outgoing<Self::Request>,
    ) -> Self::Future {
        async { js_function(task_ctx, ctx, req, tx).await }
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _tx: Outgoing<Self::Request>,
    ) -> Self::UpstreamFinishAllFuture {
        async move { Ok(ServiceContext::Complete(ctx)) }
    }
}

impl ServiceFactory for Function {
    type Future = impl Future<Output=Result<Self::Service, Self::InitError>> + Send;
    type Service = Function;
    type CtxFuture = impl Future<Output=Result<Self::Context, Self::InitError>> + Send;
    type Context = FunctionContext;
    type Config = FunctionConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Function) }
    }

    fn new_context(&self, tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        async move { new_function_context(tp, config) }
    }
}

fn create_origin<'s>(
    scope: &mut v8::HandleScope<'s>,
    filename: impl AsRef<str>,
    is_module: bool,
) -> v8::ScriptOrigin<'s> {
    let name: v8::Local<'s, v8::Value> = v8::String::new(scope, filename.as_ref()).unwrap().into();
    v8::ScriptOrigin::new(scope, name, 0, 0, false, 0, name, false, false, is_module)
}

fn module_callback<'s>(
    _context: v8::Local<'s, v8::Context>,
    _name: v8::Local<'s, v8::String>,
    _arr: v8::Local<'s, v8::FixedArray>,
    module: v8::Local<'s, v8::Module>,
) -> Option<v8::Local<'s, v8::Module>> {
    Some(module)
}

fn initialize_platform(
    config: &FunctionConfig,
) -> (UnsafeCell<v8::OwnedIsolate>, v8::Global<v8::Context>) {
    tracing::info!("toy js platform initialize...");

    static START: Once = Once::new();
    START.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
        tracing::info!("v8 platform initialized.");
    });

    let fn_name = &config.name;
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let global_context;
    {
        let handle_scope = &mut v8::HandleScope::new(&mut isolate);
        let context = v8::Context::new(handle_scope);
        global_context = v8::Global::new(handle_scope, context);

        {
            let context = v8::Local::new(handle_scope, context);
            let scope = &mut v8::ContextScope::new(handle_scope, context);
            let code = &config.code;
            let function_text =
                format!("export default function {fn_name}(request) {{ {code} return request; }}");

            let code_js = v8::String::new(scope, &function_text).unwrap();
            let origin = create_origin(scope, format!("{fn_name}.js"), true);
            let source = v8::script_compiler::Source::new(code_js, Some(&origin));
            let module = v8::script_compiler::compile_module(scope, source).unwrap();
            module.instantiate_module(scope, module_callback).unwrap();
            module.evaluate(scope).unwrap();
            let key = v8::String::new(scope, "default").unwrap();
            let obj = module
                .get_module_namespace()
                .to_object(scope)
                .unwrap()
                .get(scope, key.into())
                .unwrap();
            let key = v8::String::new(scope, fn_name).unwrap().into();
            context.global(scope).set(scope, key, obj);
        }
    }

    tracing::info!("toy js platform initialized.");
    (UnsafeCell::new(isolate), global_context)
}

pub fn new_function_context(
    _tp: ServiceType,
    config: FunctionConfig,
) -> Result<FunctionContext, ServiceError> {
    Ok(FunctionContext {
        config,
    })
}

pub async fn js_function(
    _task_ctx: TaskContext,
    ctx: FunctionContext,
    req: Frame,
    mut tx: Outgoing<Frame>,
) -> Result<ServiceContext<FunctionContext>, ServiceError> {
    thread_local! {
        static PLATFORM: OnceCell<(UnsafeCell<v8::OwnedIsolate>, v8::Global<v8::Context>)> = OnceCell::new();
    }

    let r = PLATFORM.with(|x| {
        let (isolate, global_context) = x.get_or_init(|| initialize_platform(&ctx.config));
        let isolate: &mut OwnedIsolate = unsafe { isolate.get().as_mut().unwrap_unchecked() };
        let mut handle_scope = v8::HandleScope::new(isolate);
        let context = v8::Local::new(&mut handle_scope, global_context);
        let mut context_scope = v8::ContextScope::new(&mut handle_scope, context);
        let mut scope = v8::TryCatch::new(&mut context_scope);

        let function_name = v8::String::new(&mut scope, &ctx.config.name).unwrap().into();
        let function_obj = context
            .global(&mut scope)
            .get(&mut scope, function_name)
            .unwrap();

        let func = v8::Local::<v8::Function>::try_from(function_obj).unwrap();
        let arg = codec::encode(&req, &mut scope).into();
        let args = [arg];

        if let Some(result) = func.call(&mut scope, function_obj, &args) {
            Ok(codec::decode(result, &mut scope))
        } else {
            let message = if let Some(exception) = scope.exception() {
                let exception = exception.to_object(&mut scope).unwrap();
                if let Some(message) = exception.get(&mut scope, function_name.into()) {
                    message.to_rust_string_lossy(&mut scope)
                } else {
                    exception.to_rust_string_lossy(&mut scope)
                }
            } else {
                "unknown error".to_string()
            };
            Err(ServiceError::error(message))
        }
    });

    if let Ok(v) = r {
        tx.send_ok(Frame::from_value(v)).await?;
    } else {
        return Err(r.err().unwrap());
    }

    Ok(ServiceContext::Ready(ctx))
}

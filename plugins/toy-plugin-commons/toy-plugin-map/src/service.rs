use crate::config::{
    IndexingConfig, MappingConfig, NamingConfig, PutConfig, ReindexingConfig, RemoveByIndexConfig,
    RemoveByNameConfig, RenameConfig, SingleValueConfig, ToMapConfig, ToSeqConfig, ToTransform,
    TypedConfig,
};
use crate::transform::{
    IndexingTransformer, MappingTransformer, NamingTransformer, PutTransformer,
    ReindexingTransformer, RemoveByIndexTransformer, RemoveByNameTransformer, RenameTransformer,
    SingleValueTransformer, ToMapTransformer, ToSeqTransformer, Transformer,
};
use crate::typed::convert;
use std::future::Future;
use toy_core::prelude::*;

// typed
pub struct TypedContext {
    config: TypedConfig,
}

pub fn new_typed_context(
    _tp: ServiceType,
    config: TypedConfig,
) -> Result<TypedContext, ServiceError> {
    Ok(TypedContext { config })
}

pub async fn typed(
    _task_ctx: TaskContext,
    ctx: TypedContext,
    mut req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<ServiceContext<TypedContext>, ServiceError> {
    match req.value_mut() {
        Some(v) => {
            convert(v, &ctx.config);
            tx.send_ok(req).await?;
        }
        None => (),
    }

    Ok(ServiceContext::Ready(ctx))
}

// transformer
#[derive(Clone, Debug)]
pub struct MappingContext {
    transformer: MappingTransformer,
}

pub struct NamingContext {
    transformer: NamingTransformer,
}

pub struct IndexingContext {
    transformer: IndexingTransformer,
}

pub struct ReindexingContext {
    transformer: ReindexingTransformer,
}

pub struct RenameContext {
    transformer: RenameTransformer,
}

pub struct RemoveByIndexContext {
    transformer: RemoveByIndexTransformer,
}

pub struct RemoveByNameContext {
    transformer: RemoveByNameTransformer,
}

pub struct PutContext {
    transformer: PutTransformer,
}

pub struct SingleValueContext {
    transformer: SingleValueTransformer,
}

pub struct ToMapContext {
    transformer: ToMapTransformer,
}

pub struct ToSeqContext {
    transformer: ToSeqTransformer,
}

macro_rules! transform_service {
    ($service:ident, $config: ident, $ctx: ident) => {
        #[derive(Clone, Debug)]
        pub struct $service;

        impl Service for $service {
            type Context = $ctx;
            type Request = Frame;
            type Future = impl Future<Output = Result<ServiceContext<$ctx>, ServiceError>> + Send;
            type UpstreamFinishFuture =
                impl Future<Output = Result<ServiceContext<$ctx>, ServiceError>> + Send;
            type UpstreamFinishAllFuture =
                impl Future<Output = Result<ServiceContext<$ctx>, ServiceError>> + Send;
            type Error = ServiceError;

            fn handle(
                &mut self,
                _task_ctx: TaskContext,
                ctx: Self::Context,
                mut req: Self::Request,
                mut tx: Outgoing<Self::Request, Self::Error>,
            ) -> Self::Future {
                async move {
                    match req.value_mut() {
                        Some(v) => {
                            let _ = ctx.transformer.transform(v);
                            tx.send_ok(req).await?;
                        }
                        None => (),
                    }
                    Ok(ServiceContext::Ready(ctx))
                }
            }

            fn upstream_finish(
                &mut self,
                _task_ctx: TaskContext,
                ctx: Self::Context,
                _req: Self::Request,
                _tx: Outgoing<Self::Request, Self::Error>,
            ) -> Self::UpstreamFinishFuture {
                async move { Ok(ServiceContext::Ready(ctx)) }
            }

            fn upstream_finish_all(
                &mut self,
                _task_ctx: TaskContext,
                ctx: Self::Context,
                _tx: Outgoing<Self::Request, Self::Error>,
            ) -> Self::UpstreamFinishAllFuture {
                async move { Ok(ServiceContext::Complete(ctx)) }
            }
        }

        impl ServiceFactory for $service {
            type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
            type Service = $service;
            type Context = $ctx;
            type Config = $config;
            type Request = Frame;
            type Error = ServiceError;
            type InitError = ServiceError;

            fn new_service(&self, _tp: ServiceType) -> Self::Future {
                async move { Ok($service) }
            }

            fn new_context(
                &self,
                _tp: ServiceType,
                config: Self::Config,
            ) -> Result<Self::Context, Self::InitError> {
                Ok($ctx {
                    transformer: config.into_transform(),
                })
            }
        }
    };
}

transform_service!(Mapping, MappingConfig, MappingContext);
transform_service!(Indexing, IndexingConfig, IndexingContext);
transform_service!(Reindexing, ReindexingConfig, ReindexingContext);
transform_service!(Naming, NamingConfig, NamingContext);
transform_service!(Rename, RenameConfig, RenameContext);
transform_service!(Put, PutConfig, PutContext);
transform_service!(RemoveByIndex, RemoveByIndexConfig, RemoveByIndexContext);
transform_service!(RemoveByName, RemoveByNameConfig, RemoveByNameContext);
transform_service!(SingleValue, SingleValueConfig, SingleValueContext);
transform_service!(ToMap, ToMapConfig, ToMapContext);
transform_service!(ToSeq, ToSeqConfig, ToSeqContext);

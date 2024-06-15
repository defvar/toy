use crate::config::{
    IndexingConfig, MappingConfig, NamingConfig, PutConfig, ReindexingConfig, RemoveByIndexConfig,
    RemoveByNameConfig, RenameConfig, SingleValueConfig, ToMapConfig, ToSeqConfig, ToTransform,
};
use crate::transform::{
    IndexingTransformer, MappingTransformer, NamingTransformer, PutTransformer,
    ReindexingTransformer, RemoveByIndexTransformer, RemoveByNameTransformer, RenameTransformer,
    SingleValueTransformer, ToMapTransformer, ToSeqTransformer, Transformer,
};
use std::future::Future;
use toy_core::prelude::*;

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
                mut tx: Outgoing<Self::Request>,
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

        impl ServiceFactory for $service {
            type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
            type Service = $service;
            type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
            type Context = $ctx;
            type Config = $config;
            type Request = Frame;
            type Error = ServiceError;
            type InitError = ServiceError;

            fn new_service(&self, _tp: ServiceType) -> Self::Future {
                async move { Ok($service) }
            }

            fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
                async move {
                    Ok($ctx {
                        transformer: config.into_transform(),
                    })
                }
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

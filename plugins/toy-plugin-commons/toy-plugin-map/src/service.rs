use crate::config::{
    IndexingConfig, MappingConfig, NamingConfig, ReorderConfig, ToTransform, TypedConfig,
};
use crate::typed::convert;
use crate::{Indexing, Mapping, Naming, Reorder, Transformer};
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
pub struct MappingContext {
    transformer: Mapping,
}

pub struct NamingContext {
    transformer: Naming,
}

pub struct IndexingContext {
    transformer: Indexing,
}

pub struct ReorderContext {
    transformer: Reorder,
}

macro_rules! transform {
    ($service_func:ident, $new_context_func:ident, $config: ident, $ctx: ident) => {
        pub fn $new_context_func(_tp: ServiceType, config: $config) -> Result<$ctx, ServiceError> {
            Ok($ctx {
                transformer: config.into_transform(),
            })
        }

        pub async fn $service_func(
            ctx: $ctx,
            mut req: Frame,
            mut tx: Outgoing<Frame, ServiceError>,
        ) -> Result<ServiceContext<$ctx>, ServiceError> {
            match req.value_mut() {
                Some(v) => {
                    let _ = ctx.transformer.transform(v);
                    tx.send_ok(req).await?;
                }
                None => (),
            }
            Ok(ServiceContext::Ready(ctx))
        }
    };
}

transform!(mapping, new_mapping_context, MappingConfig, MappingContext);
transform!(naming, new_naming_context, NamingConfig, NamingContext);
transform!(
    indexing,
    new_indexing_context,
    IndexingConfig,
    IndexingContext
);
transform!(reorder, new_reorder_context, ReorderConfig, ReorderContext);

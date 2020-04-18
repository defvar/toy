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
) -> Result<TypedContext, ServiceError> {
    convert(&mut req.value_mut(), &ctx.config);
    tx.send_ok(req).await?;
    Ok(ctx)
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
        ) -> Result<$ctx, ServiceError> {
            let _ = ctx.transformer.transform(&mut req.value_mut());
            tx.send_ok(req).await?;
            Ok(ctx)
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

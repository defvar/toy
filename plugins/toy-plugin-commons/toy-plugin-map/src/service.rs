use crate::config::{IndexedConfig, NamedConfig, ReorderConfig, ToTransform, TypedConfig};
use crate::transform::{Indexed, Named, Reorder, Transformer};
use crate::typed::convert;
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

pub struct NamedContext {
    transformer: Named,
}

pub struct IndexedContext {
    transformer: Indexed,
}

pub struct ReorderContext {
    transformer: Reorder,
}

macro_rules! transform {
    ($service_func:ident, $new_context_func:ident, $config: ident, $ctx: ident) => {
        pub fn $new_context_func(_tp: ServiceType, config: $config) -> Result<$ctx, ServiceError> {
            config
                .into_transform()
                .map(|transformer| $ctx { transformer })
                .ok_or(ServiceError::context_init_failed(""))
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

transform!(named, new_named_context, NamedConfig, NamedContext);
transform!(indexed, new_indexed_context, IndexedConfig, IndexedContext);
transform!(reorder, new_reorder_context, ReorderConfig, ReorderContext);

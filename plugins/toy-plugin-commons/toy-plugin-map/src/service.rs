use crate::config::{IndexedConfig, NamedConfig, ToTransform, TypedConfig};
use crate::transform::Transform;
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

// transform
pub struct TransformContext {
    t: Option<Transform>,
}

pub fn new_named_context(
    _tp: ServiceType,
    config: NamedConfig,
) -> Result<TransformContext, ServiceError> {
    transform_context(config)
}

pub fn new_indexed_context(
    _tp: ServiceType,
    config: IndexedConfig,
) -> Result<TransformContext, ServiceError> {
    transform_context(config)
}

pub async fn named(
    ctx: TransformContext,
    req: Frame,
    tx: Outgoing<Frame, ServiceError>,
) -> Result<TransformContext, ServiceError> {
    transform(ctx, req, tx).await
}

pub async fn indexed(
    ctx: TransformContext,
    req: Frame,
    tx: Outgoing<Frame, ServiceError>,
) -> Result<TransformContext, ServiceError> {
    transform(ctx, req, tx).await
}

fn transform_context<T: ToTransform>(t: T) -> Result<TransformContext, ServiceError> {
    Ok(TransformContext {
        t: t.into_transform(),
    })
}

async fn transform(
    ctx: TransformContext,
    mut req: Frame,
    mut tx: Outgoing<Frame, ServiceError>,
) -> Result<TransformContext, ServiceError> {
    let x = ctx
        .t
        .as_ref()
        .map(|x| x.transform(&mut req.value_mut()))
        .flatten();

    if let Some(v) = x {
        tx.send_ok(Frame::from_value(v)).await?;
    } else {
        // no transform ...
        tx.send_ok(req).await?;
    }
    Ok(ctx)
}

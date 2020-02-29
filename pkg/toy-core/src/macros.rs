/// Create service factory
#[macro_export(local_inner_macros)]
macro_rules! factory {
    ($f:expr, $cfg: ident, $ctx_f:expr) => {{
        || {
            service::fn_service_factory(
                |id: ServiceId| ok::<_, ServiceError>(service::fn_service(id, $f)),
                |id: ServiceId, config: $cfg| $ctx_f(id, config),
            )
        }
    }};
}

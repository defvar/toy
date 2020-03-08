/// Create service factory
#[macro_export(local_inner_macros)]
macro_rules! factory {
    ($f:expr, $cfg: ident, $ctx_f:expr) => {{
        || {
            service::fn_service_factory(
                |tp: ServiceType| ok::<_, ServiceError>(service::fn_service(tp, $f)),
                |tp: ServiceType, config: $cfg| $ctx_f(tp, config),
            )
        }
    }};
}

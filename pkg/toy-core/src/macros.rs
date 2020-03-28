/// Create service factory
#[macro_export(local_inner_macros)]
macro_rules! factory {
    ($f:expr, $cfg: ident, $ctx_f:expr) => {{
        || {
            $crate::service::fn_service_factory(
                |tp: $crate::ServiceType| {
                    $crate::service::ok::<_, $crate::error::ServiceError>(
                        $crate::service::fn_service(tp, $f),
                    )
                },
                |tp: $crate::ServiceType, config: $cfg| $ctx_f(tp, config),
            )
        }
    }};
}

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

/// Create Value::Seq
#[macro_export(local_inner_macros)]
macro_rules! seq_value {
    ($($x:expr), *) => {
        Value::from(std::vec![$($crate::data::Value::from($x)), *])
    };
}

/// Create Value::Map
#[macro_export(local_inner_macros)]
macro_rules! map_value {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(map_value!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { map_value!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = map_value!(@count $($key),*);
            let mut _map = $crate::data::Map::with_capacity(_cap);
            $(
                _map.insert($key.to_string(), $crate::data::Value::from($value));
            )*
            $crate::data::Value::from(_map)
        }
    };
}

use toy_core::prelude::*;
use toy_plugin_js::config::FunctionConfig;
use toy_plugin_js::service::{js_function, new_function_context};

#[tokio::test]
async fn test_js_function() {
    tracing_subscriber::fmt().init();

    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();
    let code = r#"
    request["payload"]["message"] = "js";
    request["payload"]["number"] += 1;
    request;
    "#;
    let config = FunctionConfig {
        name: "test".to_string(),
        code: code.to_string(),
    };
    let c = new_function_context(toy_plugin_test::dummy_service_type(), config).unwrap();

    let value = map_value! {
        "message" => "a",
        "number" => 0,
    };
    let expected = map_value! {
        "message" => "js",
        "number" => 1,
    };
    let frame = Frame::from_value(value);

    let r = js_function(task_ctx.clone(), c, frame, tx).await;
    assert!(r.is_ok());
    let r = rx.next().await.unwrap().value().cloned().unwrap();
    assert_eq!(r, expected);
}
